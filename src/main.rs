use std::env;
use std::time::Instant;
use std::thread::available_parallelism;


fn main() {
    let args: Vec<String> = env::args().collect();
    let num_calcs = *(&args[1].trim().parse::<u64>().unwrap());
    let _available_cores: u32 = available_parallelism().unwrap().get()  as u32 / 2;  // get info how many threads we can use and use half of them

    let now = Instant::now();
    let mut num_iters: u64 = 0;
    for _i in 0..num_calcs {
        num_iters += 1;
    }
    let elapsed = now.elapsed();
    let calc_per_sec: f64 = (num_calcs as f64) / (elapsed.as_secs() as f64);
    println!("Total runtime: {:.2?}", elapsed);
    println!("Calculations per second: {:.2?} seconds.", calc_per_sec);
}
