use anyhow::Result;

pub struct DioxusMcpServer;

impl DioxusMcpServer {
  pub async fn bind(&self) -> Result<std::net::SocketAddr> {
    Ok(([127, 0, 0, 1], 9000).into())
  }

  pub async fn serve(&self) -> Result<()> {
    tracing::info!("MCP server started");
    Ok(())
  }
}
