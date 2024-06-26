use std::env;
use std::io::Write;

use std::time::{Duration, Instant};
use std::{io, thread};
#[cfg(windows)]
use windows::Win32::System::Threading::{
    GetCurrentThread, THREAD_PRIORITY_TIME_CRITICAL, SetThreadAffinityMask,SetThreadPriority,
};

#[cfg(not(windows))]
fn set_thread_affinity() {
    
    // Implement the thread affinity logic for non-Windows platforms here
}
#[cfg(windows)]
fn set_thread_affinity() {
    unsafe {
        let num_cpus = num_cpus::get();
        println!("cpu numbers is: {num_cpus}");
        // let process = GetCurrentProcess();
        // let result = SetPriorityClass(process, REALTIME_PRIORITY_CLASS).is_ok();
        // if result {
        //     println!("Process priority class was successfully set to high.");
        // } else {
        //     eprintln!("Failed to set process priority class.");
        // }
        let thread = GetCurrentThread();
        let result = SetThreadPriority(thread, THREAD_PRIORITY_TIME_CRITICAL).is_ok();
        if result {
            println!("Thread priority class was successfully set to high.");
        } else {
            eprintln!("Failed to set thread priority class.");
        }
        let mask = 1 << num_cpus - 1;
        SetThreadAffinityMask(thread, mask);
    }   
}
struct PeriodInfo {
    next_period: Instant,
    period_ns: u64,
    sleep_duration: Duration,
}

impl PeriodInfo {
    fn new(period_ns: u64) -> Self {
        PeriodInfo {
            next_period: Instant::now(),
            period_ns,
            sleep_duration: Duration::from_nanos(0),
        }
    }

    fn inc_period(&mut self, time: Instant) {
        self.next_period = time + Duration::from_nanos(self.period_ns);
    }
    fn inc_period_continuous(&mut self) {
        self.next_period += Duration::from_nanos(self.period_ns);
    }

    fn wait_rest_of_period(&mut self) {
        let now = Instant::now();
        self.sleep_duration = self.next_period.duration_since(now);
        thread::sleep(self.sleep_duration);
    }
    fn spin_rest_of_period(&mut self) {
        while self.next_period > Instant::now() {
            thread::yield_now();
        }
    }
}

fn periodic_task_init(pinfo: &mut PeriodInfo) {
    pinfo.period_ns = 1_000_000; // 1ms in nanoseconds
    pinfo.next_period = Instant::now();
}

fn do_rt_task() {

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
    set_thread_affinity();
    let args: Vec<String> = env::args().collect();
    let mut mode = Mode::Spin;
    if args.len() != 2 {
        // println!("Usage: {} <mode>", args[0]);
        println!("Use default mode: spin");
    } else {
        mode = match args[1].as_str() {
            "spin" => Mode::Spin,
            "sleep" => Mode::Sleep,
            "clock" => Mode::Clock,
            _ => {
                println!("Invalid mode");
                return;
            }
        };
    }

    start_test(mode);
}
enum Mode {
    Spin,
    Sleep,
    Clock,
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Mode::Spin, Mode::Spin) => true,
            (Mode::Sleep, Mode::Sleep) => true,
            (Mode::Clock, Mode::Clock) => true,
            _ => false,
        }
    }
}
fn start_test(mode: Mode) -> ! {
    let mut pinfo = PeriodInfo::new(0);
    periodic_task_init(&mut pinfo);
    let mut stats = DurationStats::new();
    let mut count: u16 = 0;
    loop {
        let before = Instant::now();
        if mode == Mode::Clock {
            pinfo.inc_period_continuous();
        } else {
            pinfo.inc_period(before);
        }
        count += 1;
        do_rt_task();

        match mode {
            Mode::Spin => pinfo.spin_rest_of_period(),
            Mode::Sleep => pinfo.wait_rest_of_period(),
            Mode::Clock => pinfo.spin_rest_of_period(),
        }

        let duration = (Instant::now() - before).as_micros();
        stats.update(duration);
        if count == 1000 {
            stats.print_stats(duration);
            count = 0;
        }
    }
}
