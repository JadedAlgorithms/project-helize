use tokio::fs::File;
use tokio::io::AsyncSeekExt;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;
use crate::messages::LogMessage;
use tokio::io::BufReader;
use std::io;
use std::io::SeekFrom;
use chrono::Utc;
use std::time::Duration;


pub async fn tail_f(path: &str, tx: mpsc::Sender<LogMessage>) -> io::Result<()>
    {    
        let file = File::open(path).await?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::End(0)).await?;
        let mut line = String::new();
        loop{
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read > 0 {
                let log = LogMessage {
                msg_type: "log".to_string(),
                agent_id:"a1b2c3d4".to_string(),
                timestamp: Utc::now().to_rfc3339(),
                level: "info".to_string(),
                message: line.clone(),
                source_file: path.to_string(),
            };
                if tx.send(log).await.is_err() {
                    break; 
                }
                line.clear();
            }
            else{
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
