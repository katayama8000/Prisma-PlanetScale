#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello/mike").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;
    Ok(())
}
