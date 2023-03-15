use std::collections::HashMap;
use sysinfo::{ComponentExt, CpuExt, Disk, System, SystemExt, Process, ProcessExt};
#[derive(serde::Serialize, Default)]
pub struct ProcessData {
    name: String,
    memory: f64,
    pid: String,
}
#[derive(serde::Serialize, Default)]
pub struct MemoryData {
    total_memory: f64,
    total_swap: f64,
    used_memory: f64,
    used_swap: f64,
}

impl MemoryData {
    pub fn new(sysinfo: &System) -> Self {
        Self {
            total_memory: MemoryData::format_memory(sysinfo.total_memory()),
            total_swap: MemoryData::format_memory(sysinfo.total_swap()),
            used_memory: MemoryData::format_memory(sysinfo.used_memory()),
            used_swap: MemoryData::format_memory(sysinfo.used_swap()),
        }
    }

    fn format_memory(bytes: u64) -> f64 {
        return bytes as f64 / (1024. * 1024. * 1024.);
    }
}
impl ProcessData {
    pub fn new(process: &Process) -> Self {
        Self {
            name: process.name().to_string(),
            memory: MemoryData::format_memory(process.memory()),
            pid: process.pid().to_string(),
        }
    }
}
#[derive(serde::Serialize, Default)]
pub struct HostData {}
#[derive(serde::Serialize, Default)]
pub struct DiskData {
    name: String,
}
impl DiskData {
    pub fn new(disk: &Disk) -> Self {
        Self {
            name: format!("{:?}", disk),
        }
    }
}
#[derive(serde::Serialize, Default)]
pub struct SysMonitorData {
    host: HostData,
    disks: Vec<DiskData>,
    sensors: HashMap<String, f32>,
    load_avg: f64,
}
/**
 * cpu 核心数据
 */
#[derive(serde::Serialize, Default)]
pub struct CpuCoreData {
    // cpu 使用率
    usage: f32,
    // 运行频率
    frequency: f64,
}
impl CpuCoreData {
    pub fn new(usage:f32,frequency:f64)->Self{Self{usage,frequency}}
}
pub struct CpuData{
    // 芯片名称
    chip_name: String,
    // 物理核心数
    physical_core_count: usize,
    global_usage: f32,
    // cpu核心数据
    cores: Vec<CpuCoreData>,
}
impl CpuData{
    pub fn new(sys_info:&System,cores:Vec<CpuCoreData>)->Self{
        Self{
            chip_name:sys_info.global_cpu_info().brand().to_string(),
            physical_core_count:sys_info.physical_core_count().unwrap(),
            global_usage:sys_info.global_cpu_info().cpu_usage(),
            cores
        }
    }
}

#[tauri::command]
pub fn system_info()->SysMonitorData{
    let mut sys = System::new_all();
    sys.refresh_components();
    let mut sensors = HashMap::new();
    for component in sys.components() {
        sensors.insert(component.label().to_string(), component.temperature());
    }
    return SysMonitorData {
        host: HostData::default(),
        disks: vec![],
        sensors,
        load_avg: sys.load_average().one,
    };
}

#[tauri::command]
pub fn process_info() -> Vec<ProcessData> {
    let mut sys = System::new_all();
    sys.refresh_processes();
    let mut processes = vec![];
    for (pid, process) in sys.processes() {
        processes.push(ProcessData::new(&process));
    }
    processes.sort_by(|a, b| b.memory.partial_cmp(&a.memory).unwrap());
    return processes;
}
