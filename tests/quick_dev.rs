use anyhow::Result;

const URL: &'static str = "http://localhost:8000";

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(URL)?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}
