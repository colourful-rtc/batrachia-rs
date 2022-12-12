use libc::*;
use crate::{
    media_stream_track::*,
    audio_frame::*,
    video_frame::*,
    rtc_peerconnection::*,
    rtc_session_description::*,
    rtc_datachannel::*,
    rtc_icecandidate::*,
    rtc_peerconnection_configure::*,
    observer::*,
};

extern "C" {
    #[allow(improper_ctypes)]
    pub(crate) fn media_stream_audio_track_on_frame(
        track: *const RawMediaStreamTrack,
        handler: extern "C" fn(&AudioTrack, *const RawAudioFrame),
        ctx: &AudioTrack,
    );

    pub(crate) fn free_media_track(track: *const RawMediaStreamTrack);

    pub(crate) fn create_media_stream_video_track(
        label: *const c_char,
    ) -> *const RawMediaStreamTrack;
    pub(crate) fn media_stream_video_track_add_frame(
        track: *const RawMediaStreamTrack,
        frame: *const RawVideoFrame,
    );

    #[allow(improper_ctypes)]
    pub(crate) fn media_stream_video_track_on_frame(
        track: *const RawMediaStreamTrack,
        handler: extern "C" fn(&VideoTrack, *const RawVideoFrame),
        ctx: &VideoTrack,
    );

    /// The create_answer() method on the RTCPeerConnection interface creates an
    /// SDP answer to an offer received from a remote peer during the
    /// offer/answer negotiation of a WebRTC connection. The answer contains
    /// information about any media already attached to the session, codecs and
    /// options supported by the browser, and any ICE candidates already
    /// gathered. The answer is delivered to the returned Future, and should
    /// then be sent to the source of the offer to continue the negotiation
    /// process.
    pub(crate) fn rtc_create_answer(
        pc: *const RawRTCPeerConnection,
        cb: extern "C" fn(
            *const c_char,
            *const RawRTCSessionDescription,
            *mut c_void,
        ),
        ctx: *mut c_void,
    );

    /// The create_offer() method of the RTCPeerConnection interface initiates
    /// the creation of an SDP offer for the purpose of starting a new WebRTC
    /// connection to a remote peer. The SDP offer includes information about
    /// any MediaStreamTrack objects already attached to the WebRTC session,
    /// codec, and options supported by the browser, and any candidates already
    /// gathered by the ICE agent, for the purpose of being sent over the
    /// signaling channel to a potential peer to request a connection or to
    /// update the configuration of an existing connection.
    pub(crate) fn rtc_create_offer(
        pc: *const RawRTCPeerConnection,
        cb: extern "C" fn(
            *const c_char,
            *const RawRTCSessionDescription,
            *mut c_void,
        ),
        ctx: *mut c_void,
    );

    /// The RTCPeerConnection method setLocalDescription() changes the local
    /// description associated with the connection. This description specifies
    /// the properties of the local end of the connection, including the media
    /// format.
    pub(crate) fn rtc_set_local_description(
        peer: *const RawRTCPeerConnection,
        desc: *const RawRTCSessionDescription,
        callback: extern "C" fn(*const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    /// The RTCPeerConnection method setRemoteDescription() sets the specified
    /// session description as the remote peer's current offer or answer. The
    /// description specifies the properties of the remote end of the
    /// connection, including the media format.
    pub(crate) fn rtc_set_remote_description(
        peer: *const RawRTCPeerConnection,
        desc: *const RawRTCSessionDescription,
        callback: extern "C" fn(*const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    pub(crate) fn free_data_channel(channel: *const RawRTCDataChannel);

    /// Returns a string which indicates the state of the data channel's
    /// underlying data connection.
    pub(crate) fn data_channel_get_state(
        channel: *const RawRTCDataChannel,
    ) -> DataChannelState;

    /// Sends data across the data channel to the remote peer.
    pub(crate) fn data_channel_send(
        channel: *const RawRTCDataChannel,
        buf: *const u8,
        size: c_int,
    );

    #[allow(improper_ctypes)]
    pub(crate) fn data_channel_on_message(
        channel: *const RawRTCDataChannel,
        handler: extern "C" fn(&DataChannel, *const u8, u64),
        ctx: &DataChannel,
    );

    pub(crate) fn rtc_run();
    pub(crate) fn rtc_close(peer: *const RawRTCPeerConnection);

    /// The RTCPeerConnection constructor returns a newly-created
    /// RTCPeerConnection, which represents a connection between the local
    /// device and a remote peer.
    #[allow(improper_ctypes)]
    pub(crate) fn create_rtc_peer_connection(
        config: *const RawRTCPeerConnectionConfigure,
        events: *const TEvents,
        observer: *mut Observer,
    ) -> *const RawRTCPeerConnection;

    /// When a web site or app using RTCPeerConnection receives a new ICE
    /// candidate from the remote peer over its signaling channel, it
    /// delivers the newly-received candidate to the browser's ICE agent by
    /// calling RTCPeerConnection.addIceCandidate(). This adds this new
    /// remote candidate to the RTCPeerConnection's remote description,
    /// which describes the state of the remote end of the connection.
    ///
    /// If the candidate parameter is missing or a value of null is given when
    /// calling addIceCandidate(), the added ICE candidate is an
    /// "end-of-candidates" indicator. The same is the case if the value of
    /// the specified object's candidate is either missing or an empty
    /// string (""), it signals that all remote candidates have been
    /// delivered.
    ///
    /// The end-of-candidates notification is transmitted to the remote peer
    /// using a candidate with an a-line value of end-of-candidates.
    ///
    /// During negotiation, your app will likely receive many candidates which
    /// you'll deliver to the ICE agent in this way, allowing it to build up
    /// a list of potential connection methods. This is covered in more
    /// detail in the articles WebRTC connectivity and Signaling and video
    /// calling.
    pub(crate) fn rtc_add_ice_candidate(
        peer: *const RawRTCPeerConnection,
        icecandidate: *const RawRTCIceCandidate,
    ) -> bool;

    /// The RTCPeerConnection method addTrack() adds a new media track to the
    /// set of tracks which will be transmitted to the other peer.
    pub(crate) fn rtc_add_track(
        peer: *const RawRTCPeerConnection,
        track: *const RawMediaStreamTrack,
        id: *const c_char,
    );

    /// The createDataChannel() method on the RTCPeerConnection interface
    /// creates a new channel linked with the remote peer, over which any kind
    /// of data may be transmitted.
    pub(crate) fn rtc_create_data_channel(
        peer: *const RawRTCPeerConnection,
        label: *const c_char,
        options: *const RawDataChannelOptions,
    ) -> *const RawRTCDataChannel;

    pub(crate) fn free_audio_frame(frame: *const RawAudioFrame);
    
    // free the i420 video frame allocated by c.
    pub(crate) fn free_video_frame(frame: *const RawVideoFrame);

    pub(crate) fn i420_to_rgba(src: *const RawVideoFrame, dst: *mut u8) -> c_int;
}