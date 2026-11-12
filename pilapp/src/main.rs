use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use libc::{MCL_CURRENT, MCL_FUTURE, mlockall};

enum SpinTime {
    Percent(f64),
    Absolute(u64),
}
impl SpinTime {
    fn resolve(&self, period: Duration) -> Duration {
        match *self {
            SpinTime::Percent(f) => period.mul_f64(f.clamp(0.0, 100.0) / 100.0),
            SpinTime::Absolute(d) => Duration::from_nanos(d).min(period),
        }
    }
}
//run a loop with a task F. Spin for the end so thread wakes up in time
struct MainLoop<F> {
    period: Duration,
    spintime: SpinTime,
    task: F,
}
impl<F> MainLoop<F>
where
    F: FnMut() -> Result<(), Box<dyn Error>>,
{
    fn run(&mut self, runs: usize) -> Result<Vec<u128>, Box<dyn Error>> {
        let spin = self.spintime.resolve(self.period);
        let mut next = Instant::now() + self.period;
        // simply doing ::with_capacity wasn't good enough and still created regular spikes
        // due to minor page faults. vec! doesn't work either, and seems to be lazy.
        // .resize seems worth trying.
        let mut late: Vec<u128> = Vec::with_capacity(runs);
        late.resize(runs, 0);
        for i in 0..runs {
            // sleep
            sleep(next.duration_since(Instant::now() + spin));
            // spin
            while Instant::now() < next {}
            // this is just for profiling timing
            let now = Instant::now().duration_since(next).as_nanos();
            late[i] = now;

            // execute the task, access closure and call contents
            (self.task)()?;
            // don't drift over time.
            next += self.period;
        }
        Ok(late)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // de-lazify memory
    if unsafe { mlockall(MCL_CURRENT | MCL_FUTURE) != 0 } {
        panic!("Could not lock memory")
    }
    let runs = std::env::args().nth(1).expect("no runs given");
    let runs = runs.parse::<usize>().unwrap();
    let period = std::env::args().nth(2).expect("no period given");
    let period = period.parse::<u64>().unwrap();
    let mut pin = Gpio::new()?.get(17)?.into_output();
    // just to allow for logging before switching, remove from final app
    pin.set_low();
    // https://stackoverflow.com/questions/79719857/how-do-i-avoid-memory-being-swapped-in-rust
    let mut on: bool = true;
    let mut test_1000 = MainLoop {
        period: Duration::from_millis(period),
        spintime: SpinTime::Percent(100.0),
        // closure!
        task: move || -> Result<(), Box<dyn std::error::Error>> {
            match on {
                true => pin.set_high(),
                false => pin.set_low(),
            }
            on = !on;
            Ok(())
        },
    };

    let mut late = test_1000.run(runs)?;
    late.sort_unstable();
    let n = late.len();
    println!("Median: {}", late[n / 2]);
    println!("p99: {}", late[n * 99 / 100]);
    println!("Max: {}", late[n - 1]);
    Ok(())
}
