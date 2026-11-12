use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
        let mut late: Vec<u128> = Vec::with_capacity(runs);
        for _ in 0..runs {
            // sleep
            sleep(next.duration_since(Instant::now() + spin));
            // spin
            while Instant::now() < next {}
            // this is just for profiling timing
            let now = Instant::now().duration_since(next).as_nanos();
            late.push(now);
            // execute the task, access closure and call contents
            (self.task)()?;
            // don't drift over time.
            next += self.period;
        }
        Ok(late)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // form request
    // let req = Request::builder()
    //     .on_chip("/dev/gpiochip0")
    //     .with_line(17)
    //     .as_output(Value::Inactive)
    //     .request()?;
    // let mut high = false;
    let mut pin = Gpio::new()?.get(17)?.into_output();
    let mut on: bool = true;
    let mut test_1000 = MainLoop {
        period: Duration::from_millis(10),
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

    let mut late = test_1000.run(10000)?;
    late.sort_unstable();
    let n = late.len();
    println!("Median: {}", late[n / 2]);
    println!("p99: {}", late[n * 99 / 100]);
    println!("Max: {}", late[n - 1]);
    Ok(())
}
