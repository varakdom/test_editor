use bevy::prelude::{Commands, Component, Entity, Query, ResMut};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use serde::Deserialize;
use futures_lite::future;

use crate::resources::ui_state::{AssetInfo, UiState};

#[derive(Component)]
pub struct FetchAssetsTask {
    pub task: Task<Option<Vec<BvAsset>>>
}

#[derive(Debug, Deserialize)]
pub struct BvAsset {
    title: String,
    author: String,
    date: String,
    desc: String,
    tags: Vec<String>,
    downloads: i32,
    asset_path: String,
    preview_path: String,
}

pub fn prepare_asset_api_fetch(
    mut ui_state: ResMut<UiState>,
    mut cmds: Commands) {
    if !ui_state.catalog.fetch_request { return; }
    let pool = AsyncComputeTaskPool::get();

    let task = pool.spawn(async move {
        let client = reqwest::blocking::Client::new();

        let res = client.get("http://127.0.0.1:8080/assets/preview")
            .json(&serde_json::json!({"author": "KFC"}))
            .header("Content-type", "application/json")
            .send();

        if res.is_err() { return None; }

        // println!("json: {:?}", res.expect("fdp").text());
        // None
        match res.expect("Could not fetch api").json::<Vec<BvAsset>>() {
            Ok(val) => Some(val),
            Err(e) => {
                println!("prepare_asset_api_fetch, {}", e);
                None
            }
        }
    });
    cmds.spawn(FetchAssetsTask { task });
    ui_state.catalog.fetch_request = false;
}

pub fn apply_asset_api_fetch(
    mut ui_state: ResMut<UiState>,
    mut query: Query<(Entity, &mut FetchAssetsTask)>,
    mut cmds: Commands,
) {
    query.for_each_mut(|(entity, mut gen_task)| {
        if let Some(images) = future::block_on(future::poll_once(&mut gen_task.task)) {
            println!("Finished fetching !");
            if let Some(img) = images {
                for image in img {
                    println!("uri: {}", image.preview_path);
                    ui_state.catalog.queue.push_back(AssetInfo { url: image.preview_path, asset_url: image.asset_path, name: image.title });
                    ui_state.catalog.nr_fetch += 1;
                }
            }
            cmds.entity(entity).remove::<FetchAssetsTask>();
        }
        // return;
    });
}

#[derive(Component)]
pub struct AssetDownloadTask {
    pub task: Task<Option<String>>
}

pub fn prepare_asset_download(mut ui_state: ResMut<UiState>, mut cmds: Commands) {
    if ui_state.catalog.fetch_asset.is_none() { return; }
    let pool = AsyncComputeTaskPool::get();

    let url = ui_state.catalog.fetch_asset.clone().unwrap();

    println!("url: {}", url);
    let task = pool.spawn(async move {
        let client = reqwest::blocking::Client::new();

        let res = client.get("http://127.0.0.1:8080/assets/download")
            .json(&serde_json::json!({"oid": url}))// "6518e3af814cb62a15bb6b24"}))//url}))
            .header("Content-type", "application/json")
            .send();
        match res {
            Ok(res) => {
                // println!("res: {:?}", res.text());
                // let bytes = res.bytes();
                println!("res: {:?}", res.text());
                Some("".to_string())
                // AssetDownloadTask { task: Some("".to_string()) }
            }, Err(e) => { println!("Error on asset download: {}", e); None }
        }
    });
    cmds.spawn(AssetDownloadTask{ task });
    ui_state.catalog.fetch_asset = None;
}

pub fn apply_asset_download(
    // mut ui_state: ResMut<UiState>,
    mut query: Query<(Entity, &mut AssetDownloadTask)>,
    mut cmds: Commands) {
    query.for_each_mut(|(entity, mut _gen_task)| {
        cmds.entity(entity).remove::<AssetDownloadTask>();
        // return;
    });
}