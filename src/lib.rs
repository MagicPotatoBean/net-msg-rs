use std::{
    hash::{BuildHasher, DefaultHasher, Hash, Hasher},
    io::{Read, Write},
    marker::PhantomData,
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use type_hash::TypeHash;
/// 
pub struct MessageStream<SendMessageType: Serialize, RecvMessageType: DeserializeOwned> {
    send_msg_type: PhantomData<SendMessageType>,
    recv_msg_type: PhantomData<RecvMessageType>,
    connection: TcpStream,
}
impl<SendMessageType: Serialize + TypeHash, RecvMessageType: DeserializeOwned + TypeHash> MessageStream<SendMessageType, RecvMessageType> {
    pub fn new(mut stream: TcpStream) -> std::io::Result<Self> {
        let mut hasher = DefaultHasher::new();

        SendMessageType::type_hash().hash(&mut hasher);
        RecvMessageType::type_hash().hash(&mut hasher);
        stream.local_addr()?.hash(&mut hasher);
        stream.peer_addr()?.hash(&mut hasher);
        let sent_hash = hasher.finish();
        stream.write_all(&sent_hash.to_be_bytes())?;

        let mut hasher = DefaultHasher::new();
        RecvMessageType::type_hash().hash(&mut hasher);
        SendMessageType::type_hash().hash(&mut hasher);
        stream.peer_addr()?.hash(&mut hasher);
        stream.local_addr()?.hash(&mut hasher);
        let expected_hash = hasher.finish();
        let mut actual_hash = [0u8; 8];
        stream.read_exact(&mut actual_hash)?;

        if expected_hash.to_be_bytes() == actual_hash {
            Ok(Self {
                send_msg_type: PhantomData::default(),
                recv_msg_type: PhantomData::default(),
                connection: stream,
            })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Peer using a different message type (This could even be different type names, if you require this difference, use `new_unchecked()`)",
            ))
        }
    }
}
impl<SendMessageType: Serialize, RecvMessageType: DeserializeOwned> MessageStream<SendMessageType, RecvMessageType> {
    pub fn new_unchecked(stream: TcpStream) -> Self {
        Self {
            send_msg_type: PhantomData::default(),
            recv_msg_type: PhantomData::default(),
            connection: stream,
        }
    }
    pub fn send(&mut self, message: SendMessageType) -> Result<(), Box<bincode::ErrorKind>> {
        bincode::serialize_into(&mut self.connection, &message)
    }
    pub fn read(&mut self) -> Result<RecvMessageType, Box<bincode::ErrorKind>> {
        bincode::deserialize_from(&mut self.connection)
    }
}