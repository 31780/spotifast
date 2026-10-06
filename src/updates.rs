//! Self-update from GitHub releases, through fastframe-update.
//!
//! The previous release's helper installs the first release built on the
//! crate, so what it relies on stays the same: `--version` prints
//! `<command> <version>`, the new app accepts `--update-receipt` and
//! `--update-error` (stripped by [`fastframe_update::intercept`] before the
//! argument parser), and the handoff and receipt files keep their format.

pub use fastframe_update::{
    CHECK_INTERVAL, DownloadState, Installation, Kind, Prepared, Release, Source, Unsupported,
    Updater,
};
use fastframe_update::{MacConfig, ReqwestTransport, UpdateConfig};

pub const CONFIG: UpdateConfig = UpdateConfig {
    macos: MacConfig {
        bundle_ids: &["rocks.spotifast.Spotifast"],
        executable_names: &["Spotifast"],
        legacy_bundle_names: &[],
    },
    // This fork signs its releases with its own key. Verify it before
    // the updater downloads or executes any new application code.
    publisher_key: Some(include_str!("../assets/update-public-key.hex")),
    ..UpdateConfig::new(
        "31780/spotifast",
        "Spotifast",
        "spotifast",
        env!("CARGO_PKG_VERSION"),
    )
};

/// An updater on Spotifast's HTTP client, through the configured proxy.
pub fn updater(proxy: &crate::settings::ProxyConfig) -> anyhow::Result<Updater> {
    let builder = crate::http::blocking_builder(proxy).map_err(anyhow::Error::msg)?;
    Ok(Updater::new(CONFIG, ReqwestTransport::new(builder)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_config_is_valid() {
        CONFIG.validate().unwrap();
        assert_eq!(CONFIG.current_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(CONFIG.repository, "31780/spotifast");
        assert_eq!(
            CONFIG.publisher_key,
            Some(include_str!("../assets/update-public-key.hex"))
        );
    }

    // Exercise the public download API with the app's real signing key.
    // macOS requires an installed bundle, rather than a portable marker.
    #[cfg(any(target_os = "linux", windows))]
    #[test]
    fn unsigned_and_forged_manifests_never_download_executable_code() {
        use fastframe_update::{Request, Response, Transport};
        use std::io::Cursor;

        struct ReleaseServer {
            metadata: Vec<u8>,
            manifest: Vec<u8>,
        }
        impl Transport for ReleaseServer {
            fn get(&self, request: &Request<'_>) -> anyhow::Result<Response> {
                let bytes = if request.url.ends_with("/releases/tags/v9.9.9") {
                    self.metadata.clone()
                } else if request.url.ends_with("/checksums.txt") {
                    self.manifest.clone()
                } else if request.url.ends_with("/checksums.txt.sig") {
                    vec![0; 64]
                } else {
                    panic!(
                        "unsigned executable must never be requested: {}",
                        request.url
                    );
                };
                Ok(Response::ok(Cursor::new(bytes)))
            }
        }

        struct Marker(std::path::PathBuf);
        impl Drop for Marker {
            fn drop(&mut self) {
                let _ = std::fs::remove_file(&self.0);
            }
        }
        let config = UpdateConfig {
            slug: "spotifast-signature-regression",
            ..CONFIG
        };
        let exe = std::env::current_exe().unwrap();
        let path = exe
            .parent()
            .unwrap()
            .join(format!("{}-portable.txt", config.slug));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        let _marker = Marker(path);
        std::io::Write::write_all(&mut file, format!("{}-portable-v1", config.slug).as_bytes())
            .unwrap();
        drop(file);

        let (target, extension) = if cfg!(windows) {
            (format!("{}-pc-windows-msvc", std::env::consts::ARCH), "zip")
        } else {
            (
                format!("{}-unknown-linux-gnu", std::env::consts::ARCH),
                "tar.gz",
            )
        };
        let name = format!("{}-v9.9.9-{target}.{extension}", config.slug);
        let manifest = format!("{}  {name}\n", "0".repeat(64)).into_bytes();
        let asset = |name: &str, size: usize| {
            serde_json::json!({
                "name": name, "size": size,
                "browser_download_url": format!("https://github.com/{}/releases/download/v9.9.9/{name}", config.repository),
            })
        };
        for signed in [false, true] {
            let mut assets = vec![asset(&name, 1), asset("checksums.txt", manifest.len())];
            if signed {
                assets.push(asset("checksums.txt.sig", 64));
            }
            let server = ReleaseServer {
                metadata: serde_json::to_vec(&serde_json::json!({
                    "tag_name": "v9.9.9", "draft": false, "prerelease": false, "assets": assets,
                }))
                .unwrap(),
                manifest: manifest.clone(),
            };
            let error = Updater::new(config, server)
                .download(
                    &Release {
                        version: "9.9.9".into(),
                        url: "https://github.com/31780/spotifast/releases/tag/v9.9.9".into(),
                    },
                    |_, _| {},
                )
                .unwrap_err();
            let message = format!("{error:#}");
            assert!(
                message.contains(if signed {
                    "signature"
                } else {
                    "checksums.txt.sig"
                }),
                "{message}"
            );
        }
    }
}
