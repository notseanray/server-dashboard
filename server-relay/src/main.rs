use std::env;
use server_relay::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    run(&args).await?;
    Ok(())
}
