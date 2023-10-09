use futures_lite::future;
use bevy::prelude::{Commands, Component, Entity, Query, ResMut};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use egui_extras::RetainedImage;

use crate::resources::ui_state::UiState;

pub struct AssetPreview {
    pub preview: RetainedImage,
    pub name: String,
    pub asset_url: String,
}

#[derive(Component)]
pub struct PreviewTask {
    pub task: Task<Option<AssetPreview>>
}

struct ImgResource {
    /// HTTP response
    // response: ehttp::Response,

    // text: Option<String>,

    /// If set, the response was an image.
    image: Option<RetainedImage>,
}

impl ImgResource {
    fn from_response(response: ehttp::Response) -> Self {
        let content_type = response.content_type().unwrap_or_default();

        println!("Image response, {}", content_type);
        let image = if content_type.starts_with("image/") {
            println!("Creating retained image");
            RetainedImage::from_image_bytes(&response.url, &response.bytes).map_err(|err| { println!("{err}") }).ok()
        } else {
            println!("No image start");
            None
        };

        let text = response.text();

        if let Some(text) = text {
            println!("text: {}", text);
        }
        // let text = text.map(|text| text.to_owned());

        Self {
            // response,
            // text,
            image,
        }
    }
}

pub fn prepare_image_preview(
    mut ui_state: ResMut<UiState>,
    mut cmds: Commands,
) {
    let pool = AsyncComputeTaskPool::get();

    while let Some(info) = ui_state.catalog.queue.pop_front() {
        let task = pool.spawn(async move {
            let request = ehttp::Request::get(&"https://bettervoxelassetspreviewtest.s3.eu-west-2.amazonaws.com/6518e3af814cb62a15bb6b24");
            // let request = ehttp::Request::get(&"https://mir-s3-cdn-cf.behance.net/project_modules/hd/efd41a50072435.58c6cc07c277f.png");
            // let request = ehttp::Request::get(&info.url);
            // let request = "https://bettervoxelassetspreviewtest.s3.eu-west-2.amazonaws.com/6518e3af814cb62a15bb6b24";

            // println!("Fetching: {}", request.url);

            let resource = ehttp::fetch_blocking(&request)
                .map(|response| ImgResource::from_response(response));

            let r = match resource {
                Err(e) => { println!("Error: {e}"); None },
                Ok(img) => { println!("Ok: {} {}", img.image.is_some(), request.url); img.image }
            };
            if r.is_none() { println!("Asset preview is None"); return None; }
            Some(AssetPreview { preview: r.unwrap(), name: info.name, asset_url: info.asset_url })
        });
        cmds.spawn(PreviewTask { task });
    }
}

pub fn apply_image_preview(
    mut ui_state: ResMut<UiState>,
    mut query: Query<(Entity, &mut PreviewTask)>,
    mut cmds: Commands,
) {
    query.for_each_mut(|(entity, mut gen_task)| {
        if let Some(image) = future::block_on(future::poll_once(&mut gen_task.task)) {
            if let Some(img) = image {
                println!("Adding RetainedImage to cache");
                ui_state.catalog.cache.push(img);
                ui_state.catalog.nr_fetch -= 1;
            }
            cmds.entity(entity).remove::<PreviewTask>();
        }
        return;
    });
}
