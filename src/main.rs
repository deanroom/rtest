use std::io::Write;
use std::time::{Duration, Instant};
use std::{io, thread};
struct PeriodInfo {
    next_period: Instant,
    period_ns: u64,
}

impl PeriodInfo {
    fn new(period_ns: u64) -> Self {
        PeriodInfo {
            next_period: Instant::now(),
            period_ns,
        }
    }

    fn inc_period(&mut self) {
        self.next_period += Duration::from_nanos(self.period_ns);
    }

    fn wait_rest_of_period(&mut self) {
        self.inc_period();
        // let now = Instant::now();
        // let sleep_duration = self.next_period.duration_since(now);
        // thread::sleep(sleep_duration);
        while self.next_period > Instant::now() {
            // thread::yield_now();
        }
    }
}

fn periodic_task_init(pinfo: &mut PeriodInfo) {
    pinfo.period_ns = 1_000_000; // 1ms in nanoseconds
    pinfo.next_period = Instant::now();
}

fn do_rt_task() {
    thread::sleep(Duration::from_nanos(100_000));
    // Do RT stuff here.
    // Placeholder for real-time task logic
    // println!("Doing real-time task");
}

struct DurationStats {
    min: u128,
    max: u128,
    total: u128,
    count: u128,
}

impl DurationStats {
    fn new() -> Self {
        DurationStats {
            min: u128::MAX,
            max: 0,
            total: 0,
            count: 0,
        }
    }

    fn update(&mut self, duration: u128) {
        if duration < self.min {
            self.min = duration;
        }
        if duration > self.max {
            self.max = duration;
        }
        self.total += duration;
        self.count += 1;
    }

    fn average(&self) -> u128 {
        if self.count == 0 {
            0
        } else {
            self.total / self.count
        }
    }

    fn print_stats(&self, current_duration: u128) {
        print!(
            "\x1B[2K\rMin: {}μs, Max: {}μs, Avg: {}μs, Current: {}μs",
            self.min,
            self.max,
            self.average(),
            current_duration
        );
        io::stdout().flush().unwrap();
    }
}
fn main() {
    let mut pinfo = PeriodInfo::new(0);
    periodic_task_init(&mut pinfo);
    let mut stats = DurationStats::new();
    loop {
        let before = Instant::now();
        do_rt_task();
        pinfo.wait_rest_of_period();
        let after = Instant::now();
        let duration = (after - before).as_micros();
        stats.update(duration);
        stats.print_stats(duration);
    }
}
