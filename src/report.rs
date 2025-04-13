use crate::metrics::Metrics;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};

pub fn generate_report(path: &str) {
    let file = File::open(path).expect("No se pudo abrir el archivo de métricas");
    let reader = BufReader::new(file);

    let mut csv_file = File::create("metrics.csv").unwrap();

    let mut headers: Vec<String> = vec![
        "timestamp".into(),
        "cpu_temperature".into(),
        "memory_used_kb".into(),
        "memory_total_kb".into(),
        "swap_used_kb".into(),
        "cache_used_kb".into(),
        "rx_bytes".into(),
        "tx_bytes".into(),
        "connections".into(),
        "disk_reads".into(),
        "disk_writes".into(),
        "disk_read_time_ms".into(),
        "disk_write_time_ms".into(),
    ];

    let cpu_cores = 8;
    for i in 0..cpu_cores {
        headers.push(format!("cpu{}_usage", i));
        headers.push(format!("cpu{}_freq", i));
    }

    for i in 0..5 {
        headers.push(format!("top_process{}_name", i));
        headers.push(format!("top_process{}_cpu", i));
        headers.push(format!("top_process{}_memory_kb", i));
    }

    writeln!(csv_file, "{}", headers.join(",")).unwrap();

    for line in reader.lines() {
        if let Ok(l) = line {
            if let Ok(m) = serde_json::from_str::<Metrics>(&l) {
                let mut values: Vec<String> = vec![
                    m.timestamp.to_string(),
                    format!("{:.2}", m.cpu_temperature.unwrap_or(0.0)),
                    m.memory_used_kb.to_string(),
                    m.memory_total_kb.to_string(),
                    m.swap_used_kb.to_string(),
                    m.cache_used_kb.unwrap_or(0).to_string(),
                    m.network.get("wlp5s0").map_or("0".into(), |n| n.rx_bytes.to_string()),
                    m.network.get("wlp5s0").map_or("0".into(), |n| n.tx_bytes.to_string()),
                    m.active_connections.to_string(),
                ];

                let (reads, writes, read_time, write_time) = m.disk.get(0).map_or((0, 0, 0, 0), |d| {
                    (d.reads, d.writes, d.read_time_ms, d.write_time_ms)
                });

                values.extend_from_slice(&[
                    reads.to_string(),
                    writes.to_string(),
                    read_time.to_string(),
                    write_time.to_string(),
                ]);

                for i in 0..cpu_cores {
                    let usage = m.cpu.get(i).map_or(0.0, |c| c.usage_percent);
                    let freq = m.cpu.get(i).map_or(0, |c| c.frequency_mhz);
                    values.push(format!("{:.2}", usage));
                    values.push(freq.to_string());
                }

                for i in 0..5 {
                    let p = m.top_processes.get(i);
                    values.push(p.map_or("".into(), |p| p.name.clone()));
                    values.push(p.map_or("0.0".into(), |p| format!("{:.2}", p.cpu_percent)));
                    values.push(p.map_or("0".into(), |p| p.memory_kb.to_string()));
                }

                writeln!(csv_file, "{}", values.join(",")).unwrap();
            }
        }
    }

    println!("Reporte generado: metrics.csv");
}
