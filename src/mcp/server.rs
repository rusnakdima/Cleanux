use anyhow::Result;
use std::sync::Arc;

pub struct DioxusMcpServer {
    #[allow(dead_code)]
    window: Arc<tao::window::Window>,
}

impl DioxusMcpServer {
    pub fn new(window: Arc<tao::window::Window>) -> Self {
        Self { window }
    }

    pub async fn bind(&self) -> Result<std::net::SocketAddr> {
        Ok(([127, 0, 0, 1], 9000).into())
    }

    pub async fn serve(&self) -> Result<()> {
        tracing::info!("MCP server started");
        Ok(())
    }
}
