use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::sync::mpsc::sync_channel;
use std::thread;

fn main() -> io::Result<()> {
    let (tx, rx) = sync_channel::<Vec<u8>>(1024);

    let reader_thread = thread::spawn(move || {
        let stdin = io::stdin();
        let mut reader = BufReader::with_capacity(1 << 20, stdin.lock());

        // skip 2 bad lines
        let mut dump = Vec::new();
        for _ in 0..2 {
            dump.clear();
            if reader.read_until(b'\n', &mut dump).unwrap() == 0 {
                return;
            }
        }

        loop {
            let mut line = Vec::with_capacity(96);
            if reader.read_until(b'\n', &mut line).unwrap() == 0 {
                break;
            }
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    let mut switches: Vec<u64> = Vec::new();
    let mut prev: u8 = 0;
    let mut since: u64 = 0;

    //this pattern is awesome
    for line in rx {
        for group in 0..8 {
            let base = 3 + group * 9;
            for j in 0..8 {
                let bit = line[base + j] - b'0';
                if bit != prev {
                    switches.push(since);
                    since = 0;
                }
                since += 1;
                prev = bit;
            }
        }
    }

    reader_thread.join().unwrap();

    let mut out = BufWriter::new(File::create(
        "/home/noahjaye/projects/formula/pil/pilapp/jitter/out.txt",
    )?);
    for s in &switches {
        writeln!(out, "{}", s)?;
    }
    out.flush()?;
    Ok(())
}
