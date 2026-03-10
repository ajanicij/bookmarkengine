use std::{thread, time};
use rayon::prelude::*;
use rand;
use rand::RngExt;

const NUM_WORKERS: usize = 20;
const LOAD_SIZE: usize = 100;

fn do_work(worker: usize) {
    println!("Worker {} starting", worker);
    let mut rng = rand::rng();
    let delay_ms = rng.random_range(100..=2000);
    let sleep_time = time::Duration::from_millis(delay_ms);
    println!("Worker {} sleeping for {} ms", worker, delay_ms);
    thread::sleep(sleep_time);
    println!("Worker {} finishing", worker);
}

fn main() {
    let s = std::time::Instant::now();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(NUM_WORKERS) // use one thread per work slice
        .build()
        .unwrap();
    
    pool.install(|| {
        (1..=LOAD_SIZE)
            .into_par_iter()
            .for_each(|worker| do_work(worker));
    });
    println!("Work took {:?}", s.elapsed());
}
