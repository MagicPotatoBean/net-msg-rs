
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::{Read, Write},
    marker::PhantomData,
    net::TcpStream, time::Duration,
};
use serde::{de::DeserializeOwned, Serialize};
use type_hash::TypeHash;

#[derive(Debug, Clone, Copy)]
pub struct AsymmetricJointSream<SendMessageType: Serialize, RecvMessageType: DeserializeOwned, ReadWriter: Read + Write,> {
    send_msg_type: PhantomData<SendMessageType>,
    recv_msg_type: PhantomData<RecvMessageType>,
    readwriter: ReadWriter,
}
impl<SendMessageType: Serialize + TypeHash, RecvMessageType: DeserializeOwned + TypeHash, ReadWriter: Read + Write> AsymmetricJointSream<SendMessageType, RecvMessageType, ReadWriter> {
    pub fn new(mut readwriter: ReadWriter) -> std::io::Result<Self> {
        let mut hasher = DefaultHasher::new();

        SendMessageType::type_hash().hash(&mut hasher);
        RecvMessageType::type_hash().hash(&mut hasher);
        let sent_hash = hasher.finish();
        readwriter.write_all(&sent_hash.to_be_bytes())?;

        let mut hasher = DefaultHasher::new();
        RecvMessageType::type_hash().hash(&mut hasher);
        SendMessageType::type_hash().hash(&mut hasher);
        let expected_hash = hasher.finish();
        let mut actual_hash = [0u8; 8];
        readwriter.read_exact(&mut actual_hash)?;

        if expected_hash.to_be_bytes() == actual_hash {
            Ok(Self {
                send_msg_type: PhantomData::default(),
                recv_msg_type: PhantomData::default(),
                readwriter,
            })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Peer using a different message type (This could even be different type names, if you require this difference, use `new_unchecked()`)",
            ))
        }
    }
}
impl<SendMessageType: Serialize, RecvMessageType: DeserializeOwned, ReadWriter: Write + Read> AsymmetricJointSream<SendMessageType, RecvMessageType, ReadWriter> {
    pub fn new_unchecked(readwriter: ReadWriter) -> Self {
        Self {
            send_msg_type: PhantomData::default(),
            recv_msg_type: PhantomData::default(),
            readwriter,
        }
    }
    pub fn send(&mut self, message: SendMessageType) -> Result<(), Box<bincode::ErrorKind>> {
        bincode::serialize_into(&mut self.readwriter, &message)
    }
    pub fn read(&mut self) -> Result<RecvMessageType, Box<bincode::ErrorKind>> {
        bincode::deserialize_from(&mut self.readwriter)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AsymmetricSplitStream<SendMessageType: Serialize, RecvMessageType: DeserializeOwned, Reader: Read, Writer: Write> {
    send_msg_type: PhantomData<SendMessageType>,
    recv_msg_type: PhantomData<RecvMessageType>,
    reader: Reader,
    writer: Writer,
}
impl<SendMessageType: Serialize + TypeHash, RecvMessageType: DeserializeOwned + TypeHash, Reader: Read, Writer: Write> AsymmetricSplitStream<SendMessageType, RecvMessageType, Reader, Writer> {
    pub fn new(mut reader: Reader, mut writer: Writer) -> std::io::Result<Self> {
        let mut hasher = DefaultHasher::new();

        SendMessageType::type_hash().hash(&mut hasher);
        RecvMessageType::type_hash().hash(&mut hasher);
        let sent_hash = hasher.finish();
        writer.write_all(&sent_hash.to_be_bytes())?;

        let mut hasher = DefaultHasher::new();
        RecvMessageType::type_hash().hash(&mut hasher);
        SendMessageType::type_hash().hash(&mut hasher);
        let expected_hash = hasher.finish();
        let mut actual_hash = [0u8; 8];
        reader.read_exact(&mut actual_hash)?;

        if expected_hash.to_be_bytes() == actual_hash {
            Ok(Self {
                send_msg_type: PhantomData::default(),
                recv_msg_type: PhantomData::default(),
                reader,
                writer
            })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Peer using a different message type (This could even be different type names, if you require this difference, use `new_unchecked()`)",
            ))
        }
    }
}
impl<SendMessageType: Serialize, RecvMessageType: DeserializeOwned, Reader: Read, Writer: Write> AsymmetricSplitStream<SendMessageType, RecvMessageType, Reader, Writer> {
    pub fn new_unchecked(reader: Reader, writer: Writer) -> Self {
        Self {
            send_msg_type: PhantomData::default(),
            recv_msg_type: PhantomData::default(),
            reader,
            writer
        }
    }
    pub fn send(&mut self, message: SendMessageType) -> Result<(), Box<bincode::ErrorKind>> {
        bincode::serialize_into(&mut self.writer, &message)
    }
    pub fn read(&mut self) -> Result<RecvMessageType, Box<bincode::ErrorKind>> {
        bincode::deserialize_from(&mut self.reader)
    }
}

#[derive(Debug)]
pub struct AsymmetricTcpStream<SendMessageType: Serialize, RecvMessageType: DeserializeOwned> {
    send_msg_type: PhantomData<SendMessageType>,
    recv_msg_type: PhantomData<RecvMessageType>,
    connection: TcpStream,
}
impl<SendMessageType: Serialize + TypeHash, RecvMessageType: DeserializeOwned + TypeHash> AsymmetricTcpStream<SendMessageType, RecvMessageType> {
    pub fn new(mut stream: TcpStream, timeout: Duration) -> std::io::Result<Self> {
        stream.set_nonblocking(false)?;
        stream.set_read_timeout(Some(timeout))?;


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
impl<SendMessageType: Serialize, RecvMessageType: DeserializeOwned> AsymmetricTcpStream<SendMessageType, RecvMessageType> {
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
