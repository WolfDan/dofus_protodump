use std::collections::HashMap;

use anyhow::Result;
use bytes::Bytes;
use serde::Deserialize;

type GameKeyResponse = String;

#[derive(Deserialize)]
struct CytrusResponse {
    pub name: String,
    pub version: u8,
    pub games: HashMap<GameKeyResponse, GameDataResponse>,
}

#[derive(Deserialize)]
struct GameDataResponse {
    pub assets: Option<AssetsResponse>,
    #[serde(alias = "gameId")]
    pub game_id: u8,
    pub name: String,
    pub order: u8,
    pub platforms: PlatformResponse,
}

#[derive(Deserialize)]
struct AssetsResponse {
    meta: Option<VersionResponse>,
}

#[derive(Deserialize)]
struct PlatformResponse {
    darwin: Option<VersionResponse>,
    linux: Option<VersionResponse>,
    windows: Option<VersionResponse>,
}

#[derive(Deserialize, Debug)]
struct VersionResponse {
    pub beta: Option<String>,
    pub main: Option<String>,
}

pub struct Api {}

impl Api {
    pub async fn get_latest_version(game: &String, platform: &str, beta: &bool) -> Result<String> {
        let res = reqwest::get("https://cytrus.cdn.ankama.com/cytrus.json").await?;

        let response = res.json::<CytrusResponse>().await?;

        let platforms = &response.games[game].platforms;

        let platform = match platform {
            "darwin" => platforms.darwin.as_ref().unwrap(),
            "linux" => platforms.linux.as_ref().unwrap(),
            _ => platforms.windows.as_ref().unwrap(),
        };

        if *beta {
            Ok(platform.beta.clone().unwrap())
        } else {
            Ok(platform.main.clone().unwrap())
        }
    }

    pub async fn get_manifiest(
        game: &String,
        platform: &String,
        version: &String,
        beta: &bool,
    ) -> Result<Bytes> {
        let beta = if *beta {
            String::from("beta")
        } else {
            String::from("main")
        };

        let res = reqwest::get(format!(
            "https://cytrus.cdn.ankama.com/{game}/releases/{beta}/{platform}/{version}.manifest"
        ))
        .await?;

        Ok(res.bytes().await?)
    }
}
