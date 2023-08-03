use indicatif::ProgressBar;
use std::env;
use std::f64;
use std::thread::available_parallelism;
use std::time::Instant;
use sysinfo::{System, SystemExt};

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

fn add_one_loop(&n_loops: &u64) {
    for _in in 0..n_loops {
        let _ = factorial(20);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_calcs_arg: Option<&String> = args.get(1);
    let num_calcs: u64 = match num_calcs_arg {
        Some(num_calcs_arg) => num_calcs_arg.trim().parse::<u64>().unwrap(),
        None => 400000000, // runs 100 times
    };
    let num_iters: u64 = 20000;
    let total_calc: u64 = num_calcs * num_iters;
    println!(
        "Running {} calculations over {} iterations each with a total of {} calculations.",
        &num_calcs, &num_iters, &total_calc,
    );
    // get sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();
    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());
    // Number of CPUs:
    println!("Number of available threads: {}", sys.cpus().len());

    let available_cores: u64 = available_parallelism().unwrap().get() as u64; // get info how many threads we can use and use half of them
    let iter_per_core: u64 = num_calcs / available_cores;

    let now = Instant::now();

    let bar = ProgressBar::new(num_iters);
    for _i in 0..num_iters {
        let mut results = Vec::new();
        let mut threads = Vec::new();
        for _i in 0..available_cores {
            threads.push(std::thread::spawn(move || add_one_loop(&iter_per_core)));
        }
        for thread in threads {
            results.extend(thread.join());
        }
        bar.inc(1);
    }
    bar.finish();
    let elapsed = now.elapsed();
    let calc_per_sec: f64 = (total_calc as f64) / (elapsed.as_secs() as f64);
    println!("Total runtime: {:.2?}", elapsed);
    println!("Calculations per second: {:.2?} seconds.", calc_per_sec);
}
