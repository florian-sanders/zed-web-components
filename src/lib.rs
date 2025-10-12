use std::{env, fs, fs::File, io::BufReader};
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result, serde_json};

mod open_vsx;
use open_vsx::WC_LANGUAGE_SERVER_OPEN_VSX_URL;

const SERVER_PATH: &str = "wc-language-server-vsix/extension/dist/server.js";
const VERSION_PATH: &str = "wc-language-server-vsix/extension/package.json";

struct WebComponentsExtension;

impl WebComponentsExtension {
    fn read_current_version(&self) -> Option<String> {
        let file = File::open(VERSION_PATH).ok()?;
        let reader = BufReader::new(file);
        let package_json: serde_json::Value = serde_json::from_reader(reader).ok()?;
        package_json["version"].as_str().map(|s| s.to_string())
    }

    fn server_script_path(&self, language_server_id: &LanguageServerId) -> Result<String> {
        let current_version = self.read_current_version();
        let latest_version = open_vsx::fetch_latest_version()?;

        let server_exists = fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file());

        if current_version.as_deref() != Some(&latest_version) || !server_exists {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let download_url = format!(
                "{baseUrl}/{version}/file/wc-toolkit.web-components-language-server-{version}.vsix",
                baseUrl = WC_LANGUAGE_SERVER_OPEN_VSX_URL,
                version = latest_version
            );

            zed::download_file(
                &download_url,
                "wc-language-server-vsix",
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;
        }

        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for WebComponentsExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id)?;
        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: vec![Default::default()],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let initialization_options = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone());
        Ok(initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(WebComponentsExtension);
