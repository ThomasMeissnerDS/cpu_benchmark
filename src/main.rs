use std::fs;
use std::f64;
use std::env;
use std::time::Instant;
use std::thread::available_parallelism;


fn cpu_info() -> () {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap();
    let mut cpu_num = 1;

    for line in cpuinfo.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        // /proc/cpuinfo is where linux stores cpu info...
        // splitting the : so we get the record name & the value as an array
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        if key == "model name" {
            println!("CPU {}:", cpu_num);
            println!("\tCPU Name: {}", value);
            cpu_num += 1;
        } else if key == "cpu cores" {
            let cores = value.parse::<i32>().unwrap();
            println!("\tNumber of Cores: {}", cores);
        } else if key == "cpu MHz" {
            let mhz: f64 = value.parse().unwrap();
            let ghz = mhz / 1000.0;
            let ghz_rounded = (ghz * 100.0).round() / 100.0;
            println!("\tClock Speed: {:.2} GHz", ghz_rounded);
        }
    }
}


fn add_one_loop(&n_loops: &u64) -> () {
    for _in in 0..n_loops {
        let _ = 100.0 * 100.0;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_calcs_arg: Option<&String> = args.get(1);
    let num_calcs: u64 = match num_calcs_arg {
        Some(num_calcs_arg) => num_calcs_arg.trim().parse::<u64>().unwrap(),
        None => 40000000000,
    };
    cpu_info();


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
