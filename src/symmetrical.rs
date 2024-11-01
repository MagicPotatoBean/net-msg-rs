pub type SymmetricalJointStream<MessageType, ReadWriter> = crate::asymmetrical::AsymmetricalJointSream<MessageType, MessageType, ReadWriter>;
pub type SymmetricalSplitStream<MessageType, Reader, Writer> = crate::asymmetrical::AsymmetricalSplitStream<MessageType, MessageType, Reader, Writer>;
pub type SymmetricalTcpStream<MessageType> = crate::asymmetrical::AsymmetricalTcpStream<MessageType, MessageType>;

