use std::env;
use server_node::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // todo, allow self updating with args
    let args = env::args().skip(1).collect::<Vec<_>>();
    run(&args).await?;
    Ok(())
}

