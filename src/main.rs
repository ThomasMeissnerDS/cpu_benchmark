use indicatif::ProgressBar;
use std::env;
use std::f64;
use std::thread::available_parallelism;
use std::time::Instant;


fn add_one_loop(&n_loops: &u64) {
    for _in in 0..n_loops {
        let _ = 100.0 * 100.0;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_calcs_arg: Option<&String> = args.get(1);
    let num_calcs: u64 = match num_calcs_arg {
        Some(num_calcs_arg) => num_calcs_arg.trim().parse::<u64>().unwrap(),
        None => 400000000, // runs 100 times
    };
    let num_iters: u64 = 200000;
    println!(
        "Running {} calculations for {} times split across all cores.",
        &num_calcs, &num_iters
    );

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
    let calc_per_sec: f64 = (num_calcs as f64) / (elapsed.as_secs() as f64);
    println!("Total runtime: {:.2?}", elapsed);
    println!("Calculations per second: {:.2?} seconds.", calc_per_sec);
}
