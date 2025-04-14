use std::env;
use zed_extension_api::{self as zed, ContextServerId, Result};

const PACKAGE_NAME: &str = "mcp-remote";
const PACKAGE_VERSION: &str = "0.0.20";
const SERVER_PATH: &str = "node_modules/mcp-remote/dist/client.js";
const SERVER_ADDRESS: &str = "https://mcp.cloudflare.com/workers/observability/sse";

struct CloudflareModelContextServer;

impl zed::Extension for CloudflareModelContextServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &zed::Project,
    ) -> Result<zed::Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
                SERVER_ADDRESS.to_string(),
            ],
            env: vec![],
        })
    }
}

zed::register_extension!(CloudflareModelContextServer);
