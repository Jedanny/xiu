pub const WINDOW_ACKNOWLEDGEMENT_SIZE: u32 = 200000;
pub const PEER_BANDWIDTH: u32 = 200000;

pub mod PeerBandWidthLimitType {
    pub const HARD: u8 = 0;
    pub const SOFT: u8 = 1;
    pub const DYNAMIC: u8 = 2;
}

pub const FMSVER: &'static str = "FMS/3,0,1,123";
pub const CAPABILITIES: f64 = 31.0;
pub const LEVEL: &'static str = "status";

pub const OBJENCODING_AMF0: f64 = 0.0;
pub const OBJENCODING_AMF3: f64 = 3.0;

pub const STREAM_ID: f64 = 1.0;

pub const TRANSACTION_ID_CONNECT: u8 = 1;
pub const TRANSACTION_ID_CREATE_STREAM: u8 = 2;

//pub mod
pub const RTMP_LEVEL_WARNING: &'static str = "warning";
pub const RTMP_LEVEL_STATUS: &'static str = "status";
pub const RTMP_LEVEL_ERROR: &'static str = "error";