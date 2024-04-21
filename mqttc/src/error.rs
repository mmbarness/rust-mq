use std::result;
use std::io;
use std::fmt;
use thiserror::Error;
use mqtt3::{ConnectReturnCode, PacketIdentifier};
use mqtt3::MQError as MqttError;
use store::Error as StorageError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Already Connected")]
    AlreadyConnected,
    #[error("Unsupported Feature")]
    UnsupportedFeature,
    #[error("Unrecognized Packet")]
    UnrecognizedPacket,
    #[error("Connection Abort")]
    ConnectionAbort,
    #[error("Incoming Storage Absent")]
    IncommingStorageAbsent,
    #[error("Outgoing Storage Absent")]
    OutgoingStorageAbsent,
    #[error("Handshake Failed")]
    HandshakeFailed,
    #[error("Protocol Violation")]
    ProtocolViolation,
    #[error("Disconnected")]
    Disconnected,
    #[error("Timeout")]
    Timeout,
    #[error("`{0}`")]
    PacketIdentifierError(#[from] PacketIdentifierError),
    #[error("Connection Refused")]
    ConnectionRefused(#[from] ConnectReturnCode),
    #[error("`{0}`")]
    Storage(#[from] StorageError),
    #[error("`{0}`")]
    Mqtt(#[from] MqttError),
    #[error("`{0}`")]
    Io(#[from] io::Error)
}

#[derive(Debug, Error)]
pub enum PacketIdentifierError {
    UnhandledPuback(PacketIdentifier),
    UnhandledPubrec(PacketIdentifier),
    UnhandledPubrel(PacketIdentifier),
    UnhandledPubcomp(PacketIdentifier)
}

impl fmt::Display for PacketIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PacketIdentifierError::UnhandledPuback(PacketIdentifier(pi)) => fmt::write(f, format_args!("{:?}", pi)),
            PacketIdentifierError::UnhandledPubrec(PacketIdentifier(pi)) => fmt::write(f, format_args!("{:?}", pi)),
            PacketIdentifierError::UnhandledPubrel(PacketIdentifier(pi)) => fmt::write(f, format_args!("{:?}", pi)),
            PacketIdentifierError::UnhandledPubcomp(PacketIdentifier(pi)) => fmt::write(f, format_args!("{:?}", pi))
        }
    }
}
