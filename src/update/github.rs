use serde::Deserialize;
use std::path::PathBuf;

const UA: &str = concat!("RazeClicker/", env!("CARGO_PKG_VERSION"));

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub struct Found {
    pub version: String,
    pub url: String,
}

pub async fn latest() -> Result<Option<Found>, String> {
    let url = format!("https://api.github.com/repos/Sorgarso/RazeClicker/releases/latest");
    let rel: Release = reqwest::Client::new()
        .get(url)
        .header("User-Agent", UA)
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let tag = rel.tag_name.trim().trim_matches('v').to_string();

    let remote =
        semver::Version::parse(&tag).map_err(|e| format!("bad tag {}: {e}", rel.tag_name))?;
    let local = semver::Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    if remote <= local {
        return Ok(None);
    }

    let asset = rel
        .assets
        .into_iter()
        .find(|a| a.name.ends_with(".exe"))
        .ok_or("release has no .exe asset")?;

    Ok(Some(Found {
        version: remote.to_string(),
        url: asset.browser_download_url,
    }))
}

pub async fn download_and_swap(url: &str) -> Result<PathBuf, String> {
    let bytes = reqwest::Client::new()
        .get(url)
        .header("User-Agent", UA)
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    if bytes.len() < 1024 || &bytes[..2] != b"MZ" {
        return Err("downloaded file is not a valid exe".into());
    }

    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let new = exe.with_extension("new");
    let old = exe.with_extension("old");

    std::fs::write(&new, &bytes).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&old);
    std::fs::rename(&exe, &old).map_err(|e| e.to_string())?;
    if let Err(e) = std::fs::rename(&new, &exe) {
        let _ = std::fs::rename(&old, &exe);
        return Err(e.to_string());
    }
    Ok(exe)
}

pub async fn get_changelogs(url: &str, version: &str) -> Result<String, String> {
    let changelog = reqwest::Client::new()
        .get(url)
        .header("User-Agent", UA)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let header = format!("## [{}]", version);

    let section = changelog
        .split(&header)
        .nth(1)
        .ok_or_else(|| format!("Changelog for version {} not found", version))?;

    let section = section
        .lines()
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(section.trim().to_string())
}
