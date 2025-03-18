use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, ContextServerId, Result};

const PACKAGE_NAME: &str = "@cloudflare/mcp-server-cloudflare";
const PACKAGE_VERSION: &str = "0.2.0";
const SERVER_PATH: &str = "node_modules/@cloudflare/mcp-server-cloudflare/dist/index.js";

struct CloudfareModelContextServer;

#[derive(Debug, Deserialize)]
struct CloudfareModelContextServerSettings {
    cloudflare_account_id: String,
}

impl zed::Extension for CloudfareModelContextServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &zed::Project,
    ) -> Result<zed::Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let ctx_server_settings =
            ContextServerSettings::for_project("cloudfare-context-server", project)?;

        let Some(settings) = ctx_server_settings.settings else {
            Err("missing settings for cloudfare-context-server")?
        };
        let settings: CloudfareModelContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        if settings.cloudflare_account_id.is_empty() {
            Err("missing cloudflare_account_id in cloudfare-context-server settings")?;
        }

        Ok(zed::Command {
            command: "node".to_string(),
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
                "run".to_string(),
                settings.cloudflare_account_id.to_string(),
            ],
            env: vec![(
                "CLOUDFLARE_ACCOUNT_ID".into(),
                settings.cloudflare_account_id,
            )],
        })
    }
}

zed::register_extension!(CloudfareModelContextServer);
