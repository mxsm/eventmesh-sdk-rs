use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy,Deserialize,Serialize)]
pub struct EventMeshMessage {
    #[serde(rename = "bizSeqNo")]
    pub biz_seq_no: String,
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    pub topic: String,
    pub content: String,
    pub prop: HashMap<String, String>,
    #[serde(rename = "createTime")]
    pub create_time: i64,
}

impl EventMeshMessage {
    pub fn new(biz_seq_no: &str, unique_id: &str, topic: &str, content: &str, prop: HashMap<String, String>) -> Self {
        Self {
            biz_seq_no: biz_seq_no.to_string(),
            unique_id: biz_seq_no.to_string(),
            topic: topic.to_string(),
            content: content.to_string(),
            prop,
            create_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        }
    }
}
