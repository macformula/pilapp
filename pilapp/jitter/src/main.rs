// cargo build --release && sudo sigrok-cli -d fx2lafw --config samplerate=16m --samples 24M --channels D0 | cargo run
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::sync_channel;
use std::thread;

const CHUNK: usize = 1 << 20;
fn main() -> io::Result<()> {
    // sigrok child process
    let samples = std::env::args().nth(1).expect("no samples given");
    let mut child = Command::new("sigrok-cli")
        .args([
            "-d",
            "fx2lafw",
            "--config",
            "samplerate=24M",
            "--samples",
            &samples,
            "--channels",
            "D0",
            "-O",
            "binary",
        ])
        .stdout(Stdio::piped())
        .spawn()?;
    let (tx, rx) = sync_channel::<Vec<u8>>(1024);

    let reader_thread = thread::spawn(move || {
        let mut stdin = child.stdout.take().expect("no stdout");
        loop {
            let mut chunk = vec![0u8; CHUNK];
            let n = stdin.read(&mut chunk).unwrap();
            if n == 0 {
                break;
            }
            chunk.truncate(n);
            if tx.send(chunk).is_err() {
                break;
            }
        }
    });

    let mut prev: u8 = 0;
    let mut since: u64 = 0;
    let mut total: u64 = 0;
    let mut first = true;

    let mut continuous = BufWriter::new(File::create(
        "/home/noahjaye/projects/formula/pil/pilapp/jitter/continuous.txt",
    )?);

    // a guess, for now
    let mut switches = Vec::with_capacity(samples.parse::<usize>().unwrap() / 24000);
    //this pattern is awesome
    for chunk in rx {
        for &b in &chunk {
            if b & 0xFE != 0xFE {
                continue;
            }
            if first {
                prev = b;
                first = false;
            }
            if b != prev {
                writeln!(continuous, "{}", since);
                switches.push(since);
                continuous.flush()?;
                since = 0;
                if switches.len() > 5 {
                    println!()
                }
            }
            since += 1;
            prev = b;
        }
    }

    reader_thread.join().unwrap();
    Ok(())
}
