use sysinfo::System;

use serde::Serialize;

#[derive(Serialize)]
pub struct Metrics {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub process_count: usize,
}

pub fn collect_metrics() -> Metrics {
    let mut sys = System::new_all();
    sys.refresh_all();

    Metrics {
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        memory_used: sys.used_memory(),
        memory_total: sys.total_memory(),
        process_count: sys.processes().len(),
    }
}
