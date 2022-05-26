use std::{time::{SystemTime, UNIX_EPOCH}, fs};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::time::Duration;
use serde_json::json;

use actix_web::{
    get,
    http::{
        header::ContentType,
        StatusCode,
    },
    App, HttpRequest, HttpResponse, HttpServer, Result,
};

#[derive(Default, Deserialize)]
struct Config {
    servers: Vec<String>
}

impl Config {
    pub fn load() -> Self {
        let config = fs::read_to_string("./servers.json").unwrap_or_else(|_| "[]".into());
        serde_json::from_str(&config).expect("invalid config")
    }
}

lazy_static! {
    static ref CONFIG: Config = Config::load();
}

#[derive(Serialize, Debug, Default, Deserialize)]
struct Disk {
    name: String,
    fs_type: String,
    mnt_point: String,
    used_space: u64,
    total_space: u64,
    removable: bool
}

#[derive(Serialize, Debug, Default, Deserialize)]
struct Net {
    if_name: String,
    rx: u64,
    tx: u64,
    erx: u64,
    etx: u64,
    prx: u64,
    ptx: u64
}

#[derive(Serialize, Debug, Default, Deserialize)]
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

#[derive(Serialize, Debug, Default)]
struct SysinfoOut {
    timestamp: u64,
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

impl From<Sysinfo> for SysinfoOut {
    fn from(s: Sysinfo) -> Self {
        Self {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            name: s.name,
            host_name: s.host_name,
            kernel: s.kernel,
            cpu: s.cpu,
            cpu_temp: s.cpu_temp,
            cpus: s.cpus,
            core_count: s.core_count,
            swap_used: s.swap_used,
            swap_total: s.swap_total,
            memory_used: s.memory_used,
            memory_total: s.memory_total,
            disks: s.disks,
            uptime: s.uptime,
            net: s.net,
            load_avg: s.load_avg
        } 
    }
}

#[get("/data")]
async fn data(_: HttpRequest) -> Result<HttpResponse> {
    let mut server_data = Vec::with_capacity(CONFIG.servers.len());
    let client = Client::new();
    for server in &*CONFIG.servers {
        server_data.push(match client.post(format!("http://{server}/sysinfo")).json("{}").send().await {
            Ok(v) => {
                let data: SysinfoOut = v.json::<Sysinfo>().await.unwrap_or_default().into();
                json!(data).to_string()
            },
            Err(_) => continue
        });
    }
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(format!("[{}]", server_data.join(","))))
}

#[get("/data_all")]
async fn data_all(_: HttpRequest) -> Result<HttpResponse> {
    if !PathBuf::from("./server_data.json").exists() {
        let _ = fs::File::create("./server_data.json");
        return Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::plaintext())
            .body("[]"))
    }
    let all_data = fs::read_to_string("./server_data.json").unwrap_or_else(|_| "[]".into());
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(all_data))
}


pub async fn run<S: AsRef<str>>(_args: &[S]) -> anyhow::Result<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
    HttpServer::new(|| {
        App::new()
            .service(data)
            .service(data_all)
            .service(ping)
            .service(ping_all)
    })
    .bind(("127.0.0.1", 4500))?
    .workers(3)
    .run()
    .await?;
    Ok(())
}

#[get("/ping")]
async fn ping(_: HttpRequest) -> Result<HttpResponse> {    
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(format!("{{\"ping\":{}}}",         
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis())))
}

#[derive(Deserialize, Debug, Default)]
struct Ping {
    ping: u64
}

#[get("/ping_all")]
async fn ping_all(_: HttpRequest) -> Result<HttpResponse> {    
    let mut pings = Vec::with_capacity(CONFIG.servers.len());
    let client = Client::new();
    for server in &*CONFIG.servers {
        pings.push(match client.post(format!("http://{server}/ping")).json("{}").send().await {
            Ok(v) => format!("{{\"{server}\":{:#?}}}", v.json::<Ping>().await.unwrap_or_default().ping),
            Err(_) => continue
        });
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(format!("[{}]", pings.join(","))))
}
