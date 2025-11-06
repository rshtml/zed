mod html;

use html::{HtmlExtension, HTML_ID};
use std::io::Write;
use std::{env, fs};
use zed_extension_api::{self as zed, GithubRelease, LanguageServerId, Result};

const RSHTML_ID: &str = "rshtml-analyzer";
const LS_VERSION_FILE: &str = "ls_version";

struct RsHtmlExtension {
    html_extension: HtmlExtension,
    cached: bool,
}

impl RsHtmlExtension {
    fn server_exist(&self) -> bool {
        fs::metadata(RSHTML_ID).is_ok_and(|stat| stat.is_file())
    }

    fn ls_version(&self) -> String {
        fs::read_to_string(LS_VERSION_FILE).unwrap_or("v0.0.0".into())
    }

    fn write_ls_version(&self, version: &str) -> Result<()> {
        let mut file = fs::File::create(LS_VERSION_FILE).map_err(|err| err.to_string())?;
        file.write_all(version.as_bytes())
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    fn lastest_github_release(&self) -> Result<GithubRelease> {
        let github_release_options = zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        };

        zed::latest_github_release(&format!("rshtml/{}", RSHTML_ID), github_release_options)
    }

    fn check_update(&self, language_server_id: &LanguageServerId) -> Result<Option<GithubRelease>> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let github_release = self.lastest_github_release()?;

        let version = self.ls_version();

        if version == github_release.version {
            return Ok(None);
        }

        return Ok(Some(github_release));
    }

    fn download(&mut self, language_server_id: &LanguageServerId, version: &str) -> Result<()> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        let (os, arch) = zed::current_platform();

        let os = match os {
            zed::Os::Mac => "macos",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "windows",
        };

        let arch = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "x64",
            zed::Architecture::X86 => return Err("unsupported platform: x86".into()),
        };

        let url = format!(
            "https://github.com/rshtml/{}/releases/download/{}/{}-{}-{}.tar.gz",
            RSHTML_ID, version, RSHTML_ID, os, arch
        );

        zed::download_file(&url, ".", zed::DownloadedFileType::GzipTar)
            .map_err(|err| format!("{}: {}", err, url))?;

        self.write_ls_version(version)?;

        Ok(())
    }

    fn rshtml_language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command = if std::env::consts::OS == "windows" {
            format!("{}{}", language_server_id.as_ref(), ".exe")
        } else {
            language_server_id.as_ref().to_string()
        };

        let path = if let Some(path) = worktree.which(&command) {
            path
        } else {
            if self.server_exist() {
                if !self.cached {
                    self.cached = true;
                    if let Ok(github_release_option) = self.check_update(language_server_id) {
                        if let Some(github_release) = github_release_option {
                            self.download(language_server_id, &github_release.version)?;
                        }
                    } else {
                        zed::set_language_server_installation_status(
                            language_server_id,
                            &zed::LanguageServerInstallationStatus::Failed(
                                "Checking for updates failed.".to_owned(),
                            ),
                        );
                    }
                }
            } else {
                let github_release = self.lastest_github_release()?;
                self.download(language_server_id, &github_release.version)?;
                self.cached = true;
            }

            env::current_dir()
                .unwrap()
                .join(RSHTML_ID)
                .to_string_lossy()
                .to_string()
        };

        Ok(zed::Command {
            command: path,
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }
}

impl zed::Extension for RsHtmlExtension {
    fn new() -> Self {
        Self {
            html_extension: HtmlExtension::new(),
            cached: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            RSHTML_ID => self.rshtml_language_server_command(language_server_id, worktree),
            HTML_ID => self
                .html_extension
                .html_language_server_command(language_server_id, worktree),
            _ => Err("Unknown language server".into()),
        }
    }
    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        match server_id.as_ref() {
            RSHTML_ID => Ok(None),
            HTML_ID => self
                .html_extension
                .html_language_server_workspace_configuration(server_id, worktree),
            _ => Err("Unknown language server".into()),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        match server_id.as_ref() {
            RSHTML_ID => Ok(None),
            HTML_ID => self
                .html_extension
                .html_language_server_initialization_options(server_id, worktree),
            _ => Err("Unknown language server".into()),
        }
    }
}

zed::register_extension!(RsHtmlExtension);
