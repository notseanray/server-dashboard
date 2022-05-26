use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use sysinfo::{System, SystemExt, ProcessorExt, ComponentExt, DiskExt, NetworkExt};
use tide::{Request, convert::json};

#[derive(Serialize)]
struct Ping {
    pub ping: u64
}

#[derive(Serialize)]
struct Disk {
    name: String,
    fs_type: String,
    mnt_point: String,
    used_space: u64,
    total_space: u64,
    removable: bool
}

#[derive(Serialize)]
struct Net {
    if_name: String,
    rx: u64,
    tx: u64,
    erx: u64,
    etx: u64,
    prx: u64,
    ptx: u64
}

#[derive(Serialize)]
struct Sysinfo {
    name: String,
    host_name: String,
    kernel: String,
    cpu: String,
    cpu_temp: Option<f32>,
    cpus: Vec<u64>,
    core_count: usize,
    swap_used: u64,
    swap_total: u64,
    memory_used: u64,
    memory_total: u64,
    disks: Vec<Disk>,
    uptime: u64,
    net: Vec<Net>,
    load_avg: [f64; 3]
}

impl Sysinfo {
    pub fn new(sys: &mut System) -> Self {
        sys.refresh_cpu();
        let mut cpu = String::with_capacity(15);
        let core_count = sys.processors().len();
        let mut first = true;
        let mut cpus = Vec::with_capacity(core_count);
        sys.refresh_cpu();
        for core in sys.processors() {
            cpus.push(core.frequency());
            if first {
                cpu = core.brand().to_string();
                first = false;
            }
        }

        let components = sys.components();
        let cpu_temp = match components.len() {
            1.. => Some(components[0].temperature()),
            _ => None
        };

        let mut disks = Vec::with_capacity(sys.disks().len());

        for disk in sys.disks() {
            let total_space = disk.total_space();
            disks.push(Disk { 
                name: disk.name().to_string_lossy().to_string(), 
                fs_type: String::from_utf8(disk.file_system().to_vec()).unwrap_or_default(), 
                mnt_point: disk.mount_point().to_string_lossy().to_string(), 
                used_space: total_space - disk.available_space(), 
                total_space, 
                removable: disk.is_removable() 
            })
        }

        let mut net = Vec::new();

        for (if_name, data) in sys.networks() {
            let if_name = if_name.to_owned();
            // skip built in loopback
            if if_name == "lo" || if_name == "sit0" {
                continue;
            }
            net.push(Net {
                if_name,
                rx: data.total_received(),
                tx: data.total_transmitted(),
                erx: data.errors_on_received(),
                etx: data.errors_on_transmitted(),
                prx: data.total_packets_received(),
                ptx: data.total_packets_transmitted()
            });
        }

        Self {
            name: sys.name().unwrap_or_default(),
            host_name: sys.host_name().unwrap_or_default(),
            kernel: sys.kernel_version().unwrap_or_default(),
            cpu,
            cpu_temp,
            cpus,
            core_count,
            swap_used: sys.used_swap(),
            swap_total: sys.total_swap(),
            memory_used: sys.used_memory(),
            memory_total: sys.total_memory(),
            disks,
            uptime: sys.uptime(),
            net,
            load_avg: [sys.load_average().one, sys.load_average().five, sys.load_average().fifteen]
        }
    }
}

pub async fn run<S: AsRef<str>>(_args: &[S]) -> anyhow::Result<()> {
    let mut app = tide::new();
    app.at("/ping").post(handle_ping);
    app.at("/sysinfo").post(handle_sysinfo);
    app.at("/update").post(handle_update);
    app.listen("127.0.0.1:4321").await?;
    Ok(())
}

async fn handle_ping(_: Request<()>) -> tide::Result {    
    Ok(format!(
        "{{\"ping\":{}}}", 
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            )
    .into())
}

async fn handle_sysinfo(_: Request<()>) -> tide::Result {
    let mut sys = System::new_all();
    sys.refresh_all();
    Ok(json!(Sysinfo::new(&mut sys)).into())
}

async fn handle_update(_: Request<()>) -> tide::Result {
    unimplemented!();
}
