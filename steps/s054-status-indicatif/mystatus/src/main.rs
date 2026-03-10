use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() {
    let total = 100;
    let pb = ProgressBar::new(total);

    pb.set_style(
        ProgressStyle::with_template("({pos}/{len}) {msg}")
            .unwrap()
    );

    for i in 0..total {
        let url = format!("https://example{}.com", i);

        pb.set_message(format!("Fetching {}", url));

        // Simulate work.
        std::thread::sleep(Duration::from_millis(100));

        pb.inc(1);
    }

    pb.finish_with_message("Indexing complete");
}
