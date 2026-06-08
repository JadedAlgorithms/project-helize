use std::fs::read_dir;
use std::io;
use std::thread;
use crate::tailer::tail_f;
use std::time::Duration;
use std::collections::HashSet;

pub fn watch(dir: &str) -> io::Result<()> {
    let mut seen = HashSet::new();

    loop {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension() == Some(std::ffi::OsStr::new("log")) {
                if !seen.contains(&path) {
                    seen.insert(path.clone());
                    thread::spawn(move || {
                        if let Err(e) = tail_f(path.to_str().unwrap(), |line| {
                            println!("{}", line);
                        }) {
                            eprintln!("Error tailing file: {}", e);
                        }
                    });
                }
            }
        }
        thread::sleep(Duration::from_secs(2));
    }
}