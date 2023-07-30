use std::env;
use std::time::Instant;
use std::thread::available_parallelism;


fn add_one_loop(&n_loops: &u64) -> () {
    for _in in 0..n_loops {
        let _ = 1 + 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_calcs_arg: Option<&String> = args.get(1);
    let num_calcs: u64 = match num_calcs_arg {
        Some(num_calcs_arg) => num_calcs_arg.trim().parse::<u64>().unwrap(),
        None => 20000000000,
    };


    let available_cores: u64 = available_parallelism().unwrap().get()  as u64;  // get info how many threads we can use and use half of them
    let iter_per_core: u64 = num_calcs / available_cores;

    let now = Instant::now();
    let mut results = Vec::new();
    let mut threads = Vec::new();
    let range: Vec<u64> = (0..available_cores).collect();
    for _i in range {
        threads.push(std::thread::spawn(move || {
            add_one_loop(&iter_per_core)
        }));
    }
    for thread in threads {
        results.extend(thread.join());
    };
    let elapsed = now.elapsed();
    let calc_per_sec: f64 = (num_calcs as f64) / (elapsed.as_secs() as f64);
    println!("Total runtime: {:.2?}", elapsed);
    println!("Calculations per second: {:.2?} seconds.", calc_per_sec);
}
