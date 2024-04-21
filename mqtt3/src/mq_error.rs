use std::result;
use std::io;
use std::string::FromUtf8Error;
use thiserror::Error;
use byteorder;

pub type Result<T> = result::Result<T, MQError>;

#[derive(Error, Debug)]
pub enum MQError {
    #[error("Incorrect Packet Format")]
    IncorrectPacketFormat,
    #[error("Invalid Topic Path")]
    InvalidTopicPath,
    #[error("Unsupported Protocol Name")]
    UnsupportedProtocolName,
    #[error("Unsupported Protocol Version")]
    UnsupportedProtocolVersion,
    #[error("Unsupported Quality Of Service")]
    UnsupportedQualityOfService,
    #[error("Unsupported Packet Type")]
    UnsupportedPacketType,
    #[error("Unsupported Connect Return Code")]
    UnsupportedConnectReturnCode,
    #[error("Payload Size Incorrect")]
    PayloadSizeIncorrect,
    #[error("Payload Too Long")]
    PayloadTooLong,
    #[error("Payload Required")]
    PayloadRequired,
    #[error("Topic Name Must Not Contain Utf8")]
    TopicNameMustNotContainNonUtf8(#[from] FromUtf8Error),
    #[error("Topic Name Must Not Contain Wildcard")]
    TopicNameMustNotContainWildcard,
    #[error("Malformed Remaining Length")]
    MalformedRemainingLength,
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("uh oh: `{0}`")]
    Io(#[from] io::Error)
}

impl From<byteorder::Error> for MQError {
    fn from(err: byteorder::Error) -> MQError {
        match err {
            byteorder::Error::UnexpectedEOF => MQError::UnexpectedEof,
            byteorder::Error::Io(err) => MQError::Io(err)
        }
    }
}
