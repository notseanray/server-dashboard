use std::{time::{SystemTime, UNIX_EPOCH}, fs};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

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
    cpu_usage: u32,
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

#[get("/data")]
async fn data(_: HttpRequest) -> Result<HttpResponse> {
    let mut pings = Vec::with_capacity(CONFIG.servers.len());
    let client = Client::new();
    for server in &*CONFIG.servers {
        pings.push(match client.post(format!("http://{server}/sysinfo")).json("{}").send().await {
            Ok(v) => format!("{{\"{server}\":{:#?}}}", v.json::<Sysinfo>().await.unwrap_or_default()),
            Err(_) => continue
        });
    }
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(format!("[{}]", pings.join(","))))
}

pub async fn run<S: AsRef<str>>(_args: &[S]) -> anyhow::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(data)
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
