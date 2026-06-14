use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use crate::messages::LogMessage;
use crate::messages::RegisterMessage;

pub async fn run(mut rx: mpsc::Receiver<LogMessage>) {
    let stream = TcpStream::connect("127.0.0.1:9000").await;
    
    match stream {
        Ok(mut s) => {
            let reg = RegisterMessage {
                msg_type: "register".to_string(),
                agent_id: "a1b2c3d4".to_string(),
                hostname: "jaded".to_string(),
                watched_dirs: vec!["/var/log".to_string()],
                version: "0.1.0".to_string(),
            };
            let mut json = serde_json::to_string(&reg).unwrap();
            json.push('\n');
            s.write_all(json.as_bytes()).await.unwrap();
            println!("registration sent");
            while let Some(msg) = rx.recv().await {
                let mut json = serde_json::to_string(&msg).unwrap();
                json.push('\n');
                s.write_all(json.as_bytes()).await.unwrap();
            }
        }
        Err(e) => {
            println!("failed to connect: {}", e);
        }
    }
}