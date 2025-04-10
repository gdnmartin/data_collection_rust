mod metrics;
mod storage;
mod report;

use chrono::{Duration, Local};
use std::{thread, time::Duration as StdDuration};

const INTERVAL_MINUTES: i64 = 1;
const TOTAL_MINUTES: i64 = 60 * 48; // 48 horas

fn main() {
    println!("Iniciando monitor de métricas del sistema...");

    let start_time = Local::now();
    let end_time = start_time + Duration::minutes(TOTAL_MINUTES);
    let interval = StdDuration::from_secs((INTERVAL_MINUTES) as u64);

    while Local::now() < end_time {
        let timestamp = Local::now().to_rfc3339();
        let data = metrics::collect_metrics();
        println!("{}: CPU: {:.2}%, Memoria: {}MB de {}MB, Procesos: {}\n",
            timestamp,
            data.cpu_usage,
            (data.memory_used / 1024) / 1024,
            (data.memory_total / 1024) / 1024,
            data.process_count
        );
        //storage::save_metrics(&timestamp, &data);

        //println!("Datos guardados: {}", timestamp);
        thread::sleep(interval);
    }

    println!("Periodo de monitoreo terminado. Generando gráficas...");
    report::generate_report("metrics.jsonl");
}
