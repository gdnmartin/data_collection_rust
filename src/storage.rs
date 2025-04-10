use crate::metrics::Metrics;
use std::fs::OpenOptions;
use std::io::Write;
use serde_json::json;

pub fn save_metrics(timestamp: &str, data: &Metrics) {
    let json_entry = json!({
        "timestamp": timestamp,
        "metrics": data
    });

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("metrics.jsonl")
        .unwrap();

    writeln!(file, "{}", json_entry).unwrap();
}
