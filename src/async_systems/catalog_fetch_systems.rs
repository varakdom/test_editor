use futures_lite::future;
use bevy::prelude::{Commands, Component, Entity, Query, ResMut};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use egui_extras::RetainedImage;
use crate::resources::ui_state::UiState;

#[derive(Component)]
pub struct GenTask {
    pub task: Task<Option<RetainedImage>>
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
        let image = if content_type.starts_with("image/") {
            println!("Creating retained image");
            RetainedImage::from_image_bytes(&response.url, &response.bytes).map_err(|err| {println!("{err}")}).ok()
        } else {
            println!("None");
            None
        };

        // let text = response.text();
        // let text = text.map(|text| text.to_owned());

        Self {
            // response,
            // text,
            image,
        }
    }
}

pub fn prepare_gen_tasks(
    mut ui_state: ResMut<UiState>,
    mut cmds: Commands,
) {
    let pool = AsyncComputeTaskPool::get();

    while let Some(info) = ui_state.catalog.queue.pop_front() {
        let task = pool.spawn(async move {
            let request = ehttp::Request::get(&info.url);
            println!("Fetching: {}", request.url);

            let resource = ehttp::fetch_blocking(&request)
                .map(|response| ImgResource::from_response(response));

            println!("End fetch: {}", request.url);
            match resource {
                Err(e) => {println!("Error: {e}"); None},
                Ok(img) => {println!("Ok: {} {}", img.image.is_some(), request.url); img.image}
            }
        });
        cmds.spawn(GenTask{task});
    }
}

pub fn apply_gen_tasks(
    mut ui_state: ResMut<UiState>,
    mut query: Query<(Entity, &mut GenTask)>,
    mut cmds: Commands,
) {
    query.for_each_mut(|(entity, mut gen_task)| {
        if let Some(image) = future::block_on(future::poll_once(&mut gen_task.task)) {
            println!("Finished fetching !");
            if let Some(img) = image {
                println!("Adding RetainedImage to cache");
                ui_state.catalog.cache.push(img);
                ui_state.catalog.nr_fetch -= 1;
            }
            cmds.entity(entity).remove::<GenTask>();
        }
        return;
    });
}