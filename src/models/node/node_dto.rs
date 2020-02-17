use crate::models::{Uint64, uint_64::Uint64Dto};
use crate::models::node::NodeTime;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct NodeTimeDto {
    #[serde(rename = "communicationTimestamps")]
    communication_timestamps: CommunicationTimestampsDto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CommunicationTimestampsDto {
    #[serde(rename = "sendTimestamp", skip_serializing_if = "Option::is_none")]
    send_timestamp: Option<Uint64Dto>,
    #[serde(rename = "receiveTimestamp", skip_serializing_if = "Option::is_none")]
    receive_timestamp: Option<Uint64Dto>,
}

impl NodeTimeDto {
    pub(crate) fn to_struct(&self) -> NodeTime {
        let mut send = Uint64::default();
        if let Some(value) = &self.communication_timestamps.send_timestamp {
            send = value.to_struct();
        };

        let mut receive = Uint64::default();
        if let Some(value) = &self.communication_timestamps.receive_timestamp {
            receive = value.to_struct();
        };

        NodeTime {
            send_timestamp: Some(send),
            receive_timestamp: Some(receive),
        }
    }
}
