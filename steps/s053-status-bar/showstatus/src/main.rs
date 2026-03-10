

use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU64, Ordering};
use status_line::StatusLine;
use std::{thread, time};

// Define the data model representing the status of your app.
// Make sure it is Send + Sync, so it can be read and written from different threads:
struct Progress(AtomicU64);

// Define how you want to display it:
impl Display for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}% done", self.0.load(Ordering::Relaxed))
    }
}

fn main() {
    println!("Hello, world!");
    let half_second = time::Duration::from_millis(500);

    // StatusLine takes care of displaying the progress data:
    let status = StatusLine::new(Progress(AtomicU64::new(0)));   // shows 0%

    for _count in 1..100 {
        thread::sleep(half_second);
        status.0.fetch_add(1, Ordering::Relaxed);                    // shows 1%
        }
}
