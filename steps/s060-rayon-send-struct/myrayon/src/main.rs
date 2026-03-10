use std::{thread, time};
use rayon::prelude::*;
use rand;
use rand::RngExt;
use std::sync::mpsc;

const NUM_WORKERS: usize = 5;
const LOAD_SIZE: usize = 20;

#[derive(Debug)]
struct Bookmark {
    page: String,
    path: String,
}

fn do_work(worker: usize) -> Bookmark {
    println!("Worker {} starting", worker);
    let mut rng = rand::rng();
    let delay_ms = rng.random_range(100..=2000);
    let sleep_time = time::Duration::from_millis(delay_ms);
    println!("Worker {} sleeping for {} ms", worker, delay_ms);
    thread::sleep(sleep_time);
    let num = rng.random_range(100..=2000);
    let page = format!("page: num={}", num);
    let path = format!("path: num={}", num);
    let bookmark = Bookmark {
        page,
        path,
    };
    println!("Worker {} finishing", worker);
    bookmark
}

fn main() {
    let (tx, rx) = mpsc::channel::<Bookmark>();

    let s = std::time::Instant::now();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(NUM_WORKERS) // use one thread per work slice
        .build()
        .unwrap();

    // Run the receiving thread.
    thread::spawn(|| {
        for result in rx {
            println!("Got result: {:?}", result);
        }
    
    });

    // Run the sending functions in the thread pool.
    pool.install(|| {
        (1..=LOAD_SIZE)
            .into_par_iter()
            .for_each_with(tx.clone(), |tx, worker| {
                let result = do_work(worker);
                tx.send(result).unwrap();
            });
    });

    println!("Before drop");
    drop(tx);

    println!("Work took {:?}", s.elapsed());
}
