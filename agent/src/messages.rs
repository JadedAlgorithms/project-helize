#[derive(serde::Serialize)]
pub struct LogMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub agent_id: String,
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub source_file: String,
}

#[derive(serde::Serialize)]
pub struct RegisterMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub agent_id: String,
    pub hostname: String,
    pub watched_dirs: Vec<String>,
    pub version: String,
}