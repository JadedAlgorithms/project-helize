mod tailer;
mod watcher;
fn main() {
    if let Err(e) = watcher::watch("/tmp/logs") {
        eprintln!("Error: {}", e);
    }
    let log_file = "/tmp/test.log";
    println!("Watching {}...",log_file);
    if let Err(e) = tailer::tail_f(log_file, |line| {
        print!("{}",line);
    })  {
        eprintln!("Error: {}",e);
    }
}
