#[derive(Debug, PartialEq)]
pub enum RnsError {
    OutOfMemory,
    InvalidArgument,
    IncorrectSignature,
    IncorrectHash,
    CryptoError,
    PacketError,
    ConnectionError,
    LinkClosed,
    LinkNotReady,
    ChannelError,
    ChannelMessageTooBig,
    ChannelUnknownMessageType,
}
