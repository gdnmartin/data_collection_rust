use std::{collections::HashMap, fs, time::SystemTime};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Metrics {
    pub timestamp: u64,
    pub cpu: Vec<CpuCore>,
    pub cpu_temperature: Option<f32>,
    pub memory_used_kb: u64,
    pub memory_total_kb: u64,
    pub swap_used_kb: u64,
    pub cache_used_kb: Option<u64>,
    pub network: HashMap<String, NetStats>,
    pub active_connections: u64,
    pub disk: Vec<DiskStats>,
    pub top_processes: Vec<ProcessInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct CpuCore {
    pub usage_percent: f32,
    pub frequency_mhz: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NetStats {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DiskStats {
    pub device: String,
    pub reads: u64,
    pub writes: u64,
    pub read_time_ms: u64,
    pub write_time_ms: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_kb: u64,
}

pub fn collect_metrics() -> Metrics {
    let refresh = RefreshKind::new()
        .with_cpu(CpuRefreshKind::everything())
        .with_memory(MemoryRefreshKind::everything())
        .with_processes(ProcessRefreshKind::everything());

    let mut sys = System::new_with_specifics(refresh);
    sys.refresh_cpu();
    std::thread::sleep(std::time::Duration::from_secs(1));
    sys.refresh_cpu(); 
    sys.refresh_memory();
    sys.refresh_processes();

    let cpu = sys
        .cpus()
        .iter()
        .map(|core| CpuCore {
            usage_percent: core.cpu_usage(),
            frequency_mhz: core.frequency(),
        })
        .collect();

    let memory_used_kb = sys.used_memory();
    let memory_total_kb = sys.total_memory();
    let swap_used_kb = sys.used_swap();
    let cache_used_kb = read_meminfo_field("Cached:");

    let cpu_temperature = read_cpu_temp();

    let network = read_network_interfaces();
    let active_connections = count_connections();

    let disk = read_diskstats();

    let mut processes: HashMap<String, ProcessInfo> = HashMap::new();
    for process in sys.processes().values() {
        let name = process.name().to_string();
        let pid = process.pid().as_u32() as i32;
        let cpu_percent = process.cpu_usage() / 8.0;
        let memory_kb = process.memory();

        // Si el proceso ya existe, sumar los valores
        processes.entry(name.clone()).and_modify(|entry| {
            entry.cpu_percent += cpu_percent;
            entry.memory_kb += memory_kb;
        }).or_insert(ProcessInfo {
            pid,
            name,
            cpu_percent,
            memory_kb,
        });
    }

    let mut top_processes: Vec<_> = processes.into_iter().map(|(_, v)| v).collect();
    top_processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());
    top_processes.truncate(5);

    Metrics {
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        cpu,
        cpu_temperature,
        memory_used_kb,
        memory_total_kb,
        swap_used_kb,
        cache_used_kb,
        network,
        active_connections,
        disk,
        top_processes,
    }
}

fn read_cpu_temp() -> Option<f32> {
    fs::read_to_string("/sys/class/thermal/thermal_zone2/temp")
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .map(|t| t / 1000.0)
}

fn read_meminfo_field(field: &str) -> Option<u64> {
    fs::read_to_string("/proc/meminfo").ok().and_then(|contents| {
        contents
            .lines()
            .find(|line| line.starts_with(field))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|v| v.parse::<u64>().ok())
    })
}

fn read_network_interfaces() -> HashMap<String, NetStats> {
    let mut stats = HashMap::new();
    if let Ok(lines) = fs::read_to_string("/proc/net/dev") {
        for line in lines.lines().skip(2) {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 10 {
                let iface = parts[0].trim_end_matches(':');
                if iface == "wlp5s0" {
                    let rx = parts[1].parse().unwrap_or(0);
                    let tx = parts[9].parse().unwrap_or(0);
                    stats.insert(
                        iface.to_string(),
                        NetStats {
                            rx_bytes: rx,
                            tx_bytes: tx,
                        },
                    );
                }
            }
        }
    }
    stats
}

fn count_connections() -> u64 {
    let tcp = fs::read_to_string("/proc/net/tcp").map(|s| s.lines().skip(1).count()).unwrap_or(0);
    let udp = fs::read_to_string("/proc/net/udp").map(|s| s.lines().skip(1).count()).unwrap_or(0);
    (tcp + udp) as u64
}

fn read_diskstats() -> Vec<DiskStats> {
    let mut stats = vec![];
    if let Ok(lines) = fs::read_to_string("/proc/diskstats") {
        for line in lines.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 14 && parts[2] == "sda7" {
                stats.push(DiskStats {
                    device: parts[2].to_string(),
                    reads: parts[3].parse().unwrap_or(0),
                    read_time_ms: parts[6].parse().unwrap_or(0),
                    writes: parts[7].parse().unwrap_or(0),
                    write_time_ms: parts[10].parse().unwrap_or(0),
                });
            }
        }
    }
    stats
}
