use csv;
use std::error::Error;
use std::io;

use std::time::{Duration, Instant};

type Record = Vec<f32>;

fn run() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let records = rdr.deserialize().filter_map(|result| result.ok()).collect();
    Ok(records)
}

macro_rules! measure {
    ( $x:expr) => {{
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!(
            "計測開始から{}.{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_millis()
        );
        result
    }};
}

fn main() {
    measure!(run()).unwrap();
}
