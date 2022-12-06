use std::sync::Arc;
use libc::*;
use anyhow::{
    anyhow,
    Result,
};

use super::{
    RawMediaStreamTrack,
    free_media_track,
};

use crate::{
    abstracts::UintMemHeap,
    video_frame::*,
    stream_ext::*,
    base::*,
};

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    fn create_media_stream_video_track(label: *const c_char) -> *const RawMediaStreamTrack;
    fn media_stream_video_track_add_frame(track: *const RawMediaStreamTrack, frame: *const RawVideoFrame);
    fn media_stream_video_track_on_frame(
        track: *const RawMediaStreamTrack,
        handler: extern "C" fn(*mut Sinker<VideoFrame>, *const RawVideoFrame),
        ctx: *mut Sinker<VideoFrame>,
    );
}

/// The VideoTrack interface represents a single video track from
/// a MediaStreamTrack.
#[derive(Debug)]
pub struct VideoTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sink: UintMemHeap<Sinker<VideoFrame>>,
}

unsafe impl Send for VideoTrack {}
unsafe impl Sync for VideoTrack {}

impl VideoTrack {
    /// Create a new video track, may fail to create, such as
    /// insufficient memory.
    pub fn new(label: &str) -> Result<Arc<Self>> {
        let raw = unsafe { create_media_stream_video_track(to_c_str(label)?) };
        if raw.is_null() {
            Err(anyhow!("create media stream track failed!"))
        } else {
            Ok(Self::from_raw(raw))
        }
    }

    /// Push video frames to the current track, currently only
    /// supports pushing video frames in i420 format.
    ///
    /// Only valid for local video streams.
    pub fn add_frame(&self, frame: &VideoFrame) {
        unsafe {
            media_stream_video_track_add_frame(self.raw, frame.get_raw());
        }
    }

    /// Used to receive the remote video stream, the video frame of the
    /// remote video track is pushed to the receiver through the channel.
    pub fn register_sink(&self, sink: Sinker<VideoFrame>) {
        unsafe {
            media_stream_video_track_on_frame(
                self.raw,
                on_video_frame_callback,
                self.sink.set(sink),
            )
        }
    }

    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            sink: UintMemHeap::new(),
            raw,
        })
    }
}

impl Drop for VideoTrack {
    fn drop(&mut self) {
        let raw_ptr = self.raw;
        let raw = unsafe { &*raw_ptr };

        // If it is a track created locally, the label is allocated by rust
        // and needs to be freed by rust.
        if !raw.remote {
            free_cstring(raw.label);
        }
        
        unsafe { free_media_track(raw_ptr) }
    }
}

#[no_mangle]
extern "C" fn on_video_frame_callback(
    ctx: *mut Sinker<VideoFrame>,
    frame: *const RawVideoFrame,
) {
    assert!(!ctx.is_null());
    assert!(!frame.is_null());
    let frame = VideoFrame::from_raw(frame);
    unsafe { &mut *ctx }.sink.on_data(frame);
}