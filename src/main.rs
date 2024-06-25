use std::thread;
use std::time::{Duration, Instant};
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
        let now = Instant::now();
        let sleep_duration = self.next_period.duration_since(now);
        thread::sleep(sleep_duration);
        // while self.next_period > Instant::now() {
        //     thread::yield_now();
        // }
    }
}

fn periodic_task_init(pinfo: &mut PeriodInfo) {
    pinfo.period_ns = 1_000_000; // 1ms in nanoseconds
    pinfo.next_period = Instant::now();
}

fn do_rt_task() {
    // Do RT stuff here.
    // Placeholder for real-time task logic
    // println!("Doing real-time task");
}

fn main() {
    let mut pinfo = PeriodInfo::new(0);
    periodic_task_init(&mut pinfo);

    loop {
        let before = Instant::now();
        do_rt_task();
        pinfo.wait_rest_of_period();
        let after = Instant::now();
        println!("Task duration: {:?}", after.duration_since(before));
    }
}
