use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let period = Duration::from_millis(10);
    let mut next = Instant::now() + period;

    loop {
        sleep(next.duration_since(Instant::now()));
        println!("{}", Instant::now().duration_since(next).as_nanos());
        next += period;
    }
}