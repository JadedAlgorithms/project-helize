mod tailer;
mod watcher;
mod messages;
mod connection;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(connection::run(rx));
    tokio::spawn(watcher::watch("/tmp/logs", tx));

    tokio::signal::ctrl_c().await.unwrap();
    println!("shutting down");
}