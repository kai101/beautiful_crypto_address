mod hash_task;

use std::thread;
use num_cpus;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Beautiful Cryto Address")
        .version("1.0")
        .author("Leon Yap <leon.yap@gmail.com>")
        .about("Discover special hash address")
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("Number of hash to perform"))
        .get_matches();

    let count: usize = matches
        .value_of("num").unwrap_or("1000")
        .parse().expect("number is not integer");
    let mut handles = Vec::new();
    
    for _ in 0..num_cpus::get() {
        let handle = thread::spawn(move || {
            hash_task::run(count/num_cpus::get())
        });
        handles.push(handle);
    }
    let mut result = 0;
    for handle in handles {
        let num = handle.join().unwrap();
        result += num
    }

    println!("total perf {}", result)
}