use std::{env, fs::{File, OpenOptions}, io::{self, Read, Write}, time::{SystemTime, UNIX_EPOCH, Duration}};
use chrono::Local;

mod metrics;
mod storage;
mod report;

const TOTAL_HOURS: u64 = 48;
const START_FILE: &str = "start_time.txt";

fn get_or_create_start_time() -> io::Result<SystemTime> {
    let start_file_path = env::current_dir()?.join(START_FILE);

    if start_file_path.exists() {
        let mut content = String::new();
        File::open(&start_file_path)?.read_to_string(&mut content)?;
        let start_secs = content.trim().parse::<u64>().unwrap_or(0);
        Ok(UNIX_EPOCH + Duration::from_secs(start_secs))
    } else {
        let now = SystemTime::now();
        let start_secs = now.duration_since(UNIX_EPOCH).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.as_secs();
        let mut file = OpenOptions::new().create(true).write(true).open(&start_file_path)?;
        writeln!(file, "{}", start_secs)?;
        Ok(now)
    }
}

fn main() {
    let now = SystemTime::now();
    let start_time = get_or_create_start_time().expect("No se pudo obtener o crear start_time.txt");

    let elapsed = now.duration_since(start_time).unwrap_or(Duration::ZERO);
    let limit = Duration::from_secs(TOTAL_HOURS * 3600);

    if elapsed < limit {
        let timestamp = Local::now().to_rfc3339();
        let metrics = metrics::collect_metrics();
        storage::save_metrics(&metrics);  
        storage::clean_old_metrics();     
        println!("Métricas guardadas: {}", timestamp);
    } else {
        println!("Tiempo completado. Generando reporte...");
        report::generate_report("metrics.jsonl");
        println!("Reporte generado.");
    }
}
