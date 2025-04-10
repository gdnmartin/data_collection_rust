use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use serde_json::Value;

pub fn generate_report(path: &str) {
    let file = File::open(path).expect("No se pudo abrir el archivo de métricas");
    let reader = BufReader::new(file);

    let mut csv_file = File::create("metrics.csv").unwrap();
    writeln!(csv_file, "timestamp,cpu_usage,memory_used,process_count").unwrap();

    for line in reader.lines() {
        if let Ok(l) = line {
            let v: Value = serde_json::from_str(&l).unwrap();
            let ts = v["timestamp"].as_str().unwrap();
            let m = &v["metrics"];
            let row = format!(
                "{},{},{},{}",
                ts,
                m["cpu_usage"].as_f64().unwrap(),
                m["memory_used"].as_u64().unwrap(),
                m["process_count"].as_u64().unwrap()
            );
            writeln!(csv_file, "{}", row).unwrap();
        }
    }

    println!("Archivo CSV generado: metrics.csv");
    // Puedes agregar aquí integración con `plotters` o scripts Python para graficar
}
