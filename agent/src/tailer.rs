use std::fs::File;
use std::io::{self,BufRead,BufReader,Seek,SeekFrom};
use std::thread;
use std::time::Duration;

pub fn tail_f<F>(path: &str, mut on_line: F) -> io::Result<()>
where
    F: FnMut(String) {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::End(0))?;
        let mut line = String::new();
        loop{
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read > 0 {
                on_line(line.clone());
                line.clear();
            }
            else{
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
