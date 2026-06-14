use std::fs::read_dir;
use std::io;
use std::collections::HashSet;
use std::time::Duration;
use tokio::sync::mpsc;
use crate::messages::LogMessage;
use crate::tailer::tail_f;

pub async fn watch(dir: &str, tx: mpsc::Sender<LogMessage>) -> io::Result<()> {
    let mut seen = HashSet::new();
    loop{
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension() == Some(std::ffi::OsStr::new("log")) {
                if !seen.contains(&path){
                    seen.insert(path.clone());
                    let tx_clone = tx.clone();
                    tokio::spawn(async move{
                        if let Err(e) = tail_f(path.to_str().unwrap(),tx_clone).await {
                            eprintln!("Error fetching the file: {}", e);
                        }
                    });

                }
            }
            tokio::time::sleep(Duration::from_millis(2000)).await;
        }
    }

}