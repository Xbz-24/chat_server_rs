use crate::shared::message::Message;
//use serde::{Serialize, Deserialize};

pub fn serialize_message(message: &Message) -> Result<Vec<u8>, Box<dyn std::error::Error>> {


    let encoded = bincode::serialize(message)?;

    Ok(encoded)

}


pub fn deserialize_message(data: &[u8]) -> Result<Message, Box<dyn std::error::Error>> {

    let decoded: Message = bincode::deserialize(data)?;


    Ok(decoded)

}
