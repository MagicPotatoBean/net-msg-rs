
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::{Read, Write},
    marker::PhantomData,
    net::TcpStream,
};
use serde::{de::DeserializeOwned, Serialize};
use type_hash::TypeHash;


pub struct SymmetricalJointStream<MessageType: Serialize + DeserializeOwned, ReadWriter: Read + Write,> {
    msg_type: PhantomData<MessageType>,
    readwriter: ReadWriter,
}
impl<MessageType: Serialize + TypeHash + DeserializeOwned, ReadWriter: Read + Write> SymmetricalJointStream<MessageType, ReadWriter> {
    pub fn new(mut readwriter: ReadWriter) -> std::io::Result<Self> {
        let mut hasher = DefaultHasher::new();

        MessageType::type_hash().hash(&mut hasher);
        MessageType::type_hash().hash(&mut hasher);
        let sent_hash = hasher.finish();
        readwriter.write_all(&sent_hash.to_be_bytes())?;

        let mut hasher = DefaultHasher::new();
        MessageType::type_hash().hash(&mut hasher);
        MessageType::type_hash().hash(&mut hasher);
        let expected_hash = hasher.finish();
        let mut actual_hash = [0u8; 8];
        readwriter.read_exact(&mut actual_hash)?;

        if expected_hash.to_be_bytes() == actual_hash {
            Ok(Self {
                msg_type: PhantomData::default(),
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
impl<MessageType: Serialize + DeserializeOwned, ReadWriter: Write + Read> SymmetricalJointStream<MessageType, ReadWriter> {
    pub fn new_unchecked(readwriter: ReadWriter) -> Self {
        Self {
            msg_type: PhantomData::default(),
            readwriter,
        }
    }
    pub fn send(&mut self, message: MessageType) -> Result<(), Box<bincode::ErrorKind>> {
        bincode::serialize_into(&mut self.readwriter, &message)
    }
    pub fn read(&mut self) -> Result<MessageType, Box<bincode::ErrorKind>> {
        bincode::deserialize_from(&mut self.readwriter)
    }
}

pub struct SymmetricalSplitStream<MessageType: Serialize + DeserializeOwned, Reader: Read, Writer: Write> {
    msg_type: PhantomData<MessageType>,
    reader: Reader,
    writer: Writer,
}
impl<MessageType: Serialize + TypeHash + DeserializeOwned, Reader: Read, Writer: Write> SymmetricalSplitStream<MessageType, Reader, Writer> {
    pub fn new(mut reader: Reader, mut writer: Writer) -> std::io::Result<Self> {
        let mut hasher = DefaultHasher::new();

        MessageType::type_hash().hash(&mut hasher);
        MessageType::type_hash().hash(&mut hasher);
        let sent_hash = hasher.finish();
        writer.write_all(&sent_hash.to_be_bytes())?;

        let mut hasher = DefaultHasher::new();
        MessageType::type_hash().hash(&mut hasher);
        MessageType::type_hash().hash(&mut hasher);
        let expected_hash = hasher.finish();
        let mut actual_hash = [0u8; 8];
        reader.read_exact(&mut actual_hash)?;

        if expected_hash.to_be_bytes() == actual_hash {
            Ok(Self {
                msg_type: PhantomData::default(),
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
impl<MessageType: Serialize + DeserializeOwned, Reader: Read, Writer: Write> SymmetricalSplitStream<MessageType, Reader, Writer> {
    pub fn new_unchecked(reader: Reader, writer: Writer) -> Self {
        Self {
            msg_type: PhantomData::default(),
            reader,
            writer
        }
    }
    pub fn send(&mut self, message: MessageType) -> Result<(), Box<bincode::ErrorKind>> {
        bincode::serialize_into(&mut self.writer, &message)
    }
    pub fn read(&mut self) -> Result<MessageType, Box<bincode::ErrorKind>> {
        bincode::deserialize_from(&mut self.reader)
    }
}

pub struct SymmetricalTcpStream<MessageType: Serialize + DeserializeOwned> {
    msg_type: PhantomData<MessageType>,
    connection: TcpStream,
}
impl<MessageType: Serialize + TypeHash + DeserializeOwned> SymmetricalTcpStream<MessageType> {
    pub fn new(mut stream: TcpStream) -> std::io::Result<Self> {
        let mut hasher = DefaultHasher::new();

        MessageType::type_hash().hash(&mut hasher);
        MessageType::type_hash().hash(&mut hasher);
        stream.local_addr()?.hash(&mut hasher);
        stream.peer_addr()?.hash(&mut hasher);
        let sent_hash = hasher.finish();
        stream.write_all(&sent_hash.to_be_bytes())?;

        let mut hasher = DefaultHasher::new();
        MessageType::type_hash().hash(&mut hasher);
        MessageType::type_hash().hash(&mut hasher);
        stream.peer_addr()?.hash(&mut hasher);
        stream.local_addr()?.hash(&mut hasher);
        let expected_hash = hasher.finish();
        let mut actual_hash = [0u8; 8];
        stream.read_exact(&mut actual_hash)?;

        if expected_hash.to_be_bytes() == actual_hash {
            Ok(Self {
                msg_type: PhantomData::default(),
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
impl<MessageType: Serialize + DeserializeOwned> SymmetricalTcpStream<MessageType> {
    pub fn new_unchecked(stream: TcpStream) -> Self {
        Self {
            msg_type: PhantomData::default(),
            connection: stream,
        }
    }
    pub fn send(&mut self, message: MessageType) -> Result<(), Box<bincode::ErrorKind>> {
        bincode::serialize_into(&mut self.connection, &message)
    }
    pub fn read(&mut self) -> Result<MessageType, Box<bincode::ErrorKind>> {
        bincode::deserialize_from(&mut self.connection)
    }
}
