pub type SymmetricJointStream<MessageType, ReadWriter> = crate::asymmetric::AsymmetricJointSream<MessageType, MessageType, ReadWriter>;
pub type SymmetricSplitStream<MessageType, Reader, Writer> = crate::asymmetric::AsymmetricSplitStream<MessageType, MessageType, Reader, Writer>;
pub type SymmetricTcpStream<MessageType> = crate::asymmetric::AsymmetricTcpStream<MessageType, MessageType>;

