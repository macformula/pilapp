use std::thread::sleep;
use std::time::{Duration, Instant};

enum SpinTime {
    Percent(f64),
    Absolute(u128),
}
impl SpinTime {
    fn resolve(&self, period: Duration) -> Duration {
        match *self {
            SpinTime::Percent(f) => period.mul_f64(f.clamp(0.0, 100.0) / 100.0),
            SpinTime::Absolute(d) => Duration::from_nanos_u128(d.min(period.as_nanos())),
        }
    }
}
struct MainLoop {
    period: Duration,
    spintime: SpinTime,
}
impl MainLoop {
    fn run(&self, runs: usize) -> Vec<u128> {
        let spin = self.spintime.resolve(self.period);
        let mut next = Instant::now() + self.period;
        let mut late: Vec<u128> = Vec::with_capacity(runs);
        for _ in 0..runs {
            sleep(next.duration_since(Instant::now() + spin));
            while Instant::now() < next {}
            let now = Instant::now().duration_since(next).as_nanos();
            next += self.period;
            late.push(now);
        }
        late
    }
}
fn main() {
    let test_1000 = MainLoop {
        period: Duration::from_millis(10),
        spintime: SpinTime::Percent(10.0),
    };
    let mut late = test_1000.run(10000);
    late.sort_unstable();
    println!("Median: {}", late[1000 / 2]);
    println!("p99: {}", late[99 * 1000 / 100]);
    println!("Max: {}", late[1000 - 2]);
}
