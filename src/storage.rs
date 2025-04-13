use std::fs::OpenOptions;
use std::io::{BufReader, Write, BufRead};
use std::fs::File;
use std::time::SystemTime;
use crate::metrics::Metrics;

const MAX_AGE_SECONDS: u64 = 60 * 60 * 24 * 2;


pub fn save_metrics(metrics: &Metrics) {
    let json = serde_json::to_string(metrics).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("metrics.jsonl")
        .expect("No se pudo abrir o crear el archivo metrics.jsonl");

    writeln!(file, "{}", json).expect("No se pudo escribir en el archivo");
}

pub fn clean_old_metrics() {
    let file = File::open("metrics.jsonl").expect("No se pudo abrir el archivo");
    let reader = BufReader::new(file);

    let mut new_metrics = Vec::new();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    for line in reader.lines() {
        if let Ok(json_line) = line {
            if let Ok(entry) = serde_json::from_str::<Metrics>(&json_line) {
                if now - entry.timestamp <= MAX_AGE_SECONDS {
                    new_metrics.push(entry);
                }
            }
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("metrics.jsonl")
        .expect("No se pudo abrir o crear el archivo");

    for metric in new_metrics {
        let json = serde_json::to_string(&metric).unwrap();
        writeln!(file, "{}", json).expect("No se pudo escribir en el archivo");
    }
}
