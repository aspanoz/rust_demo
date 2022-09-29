use async_std::channel::Sender;
use fs_extra::file::{move_file, CopyOptions};
use rand::{thread_rng, Rng};
use rexiv2::Metadata;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tide_websockets_sink::Message;

mod media;
use media::{Category, Media};

#[derive(Debug, Clone)]
pub struct State {
    pub media: Vec<Media>,
    pub next_id: u32,
    pub connections: HashMap<u32, Sender<Message>>,
    pub config: super::config::Config,
}
pub type Db = Arc<Mutex<State>>;

impl State {
    pub fn get_rand_media_id(&mut self) -> String {
        let idx = thread_rng().gen_range(0..self.media.len());
        let item = self.get_media_item(idx as usize).clone();

        format!(
            r#"{{ "media": {{ "idx": {}, "type": "{}", "width": {}, "height": {}, "like": {} }} }}"#,
            idx,
            item.category,
            item.width.unwrap_or(0),
            item.height.unwrap_or(0),
            item.data.unwrap_or("".to_string()).contains("0")
        )
    }

    pub fn get_media_item(&mut self, idx: usize) -> Media {
        let mut item = self.media[idx as usize].clone();
        if item.data.is_some() {
            let item = item.clone();
            return item;
        }
        let path = item.path.clone();

        match Metadata::new_from_path(item.path.clone()) {
            Ok(meta) if meta.get_media_type() == Ok(rexiv2::MediaType::Gif) => {
                item.width = Some(meta.get_pixel_width());
                item.height = Some(meta.get_pixel_height());
            }
            Ok(meta) if !meta.supports_exif() => {
            }
            Err(e) => println!("ERROR {:?}", e),

            Ok(meta) => {
                let is_parsed = meta.get_tag_string("Exif.Image.Software")
                    == Ok("Home Media Manager".to_string());
                if !meta.has_exif() || !is_parsed {
                    if !is_parsed {
                        meta.clear();
                    }

                    if meta
                        .set_tag_string("Exif.Image.Software", "Home Media Manager")
                        .is_ok()
                    {
                        meta.save_to_file(path)
                            .expect("Couldn't save metadata to file");
                    }
                }

                let data = match meta.get_tag_string("Exif.Image.ImageID") {
                    Ok(value) => value,
                    _ => String::from(""),
                };

                item.data = Some(data);
                item.width = Some(meta.get_pixel_width());
                item.height = Some(meta.get_pixel_height());
            }
        };
        let item = item.clone();
        return item;
    }

    pub fn set_like_by_id(&mut self, idx: usize) -> String {
        let mut item = self.get_media_item(idx as usize);

        if let Ok(meta) = Metadata::new_from_path(item.path.clone()) {
            let data = item.data.unwrap_or("".to_string());
            let data = format!("0{}", data);
            item.data = Some(data.clone());
            if meta.set_tag_string("Exif.Image.ImageID", &data).is_ok() {
                meta.save_to_file(item.path)
                    .expect("Couldn't save metadata to file");
                println!("set like");
            }
        }

        format!(
            r#"{{ "media": {{ "idx": {}, "type": "{}", "width": {}, "height": {}, "like": {} }} }}"#,
            idx,
            item.category,
            item.width.unwrap_or(0),
            item.height.unwrap_or(0),
            item.data.unwrap_or("".to_string()).contains("0")
        )
    }

    pub fn get_file_path(&self, id: usize) -> String {
        self.media
            .get(id)
            .expect("Unable to media path by id")
            .path
            .clone()
    }

    pub fn move_to_trash(&mut self, id: usize) {
        let trashed = self.media.remove(id);
        let new_path = Path::new(&trashed.path)
            .file_name()
            .expect("Unable to get file name");
        // проверку на то, что такой файл уже есть
        let new_path = Path::new(&self.config.trash).join(new_path);
        let options = CopyOptions::new();
        move_file(trashed.path, new_path, &options).expect("Unable to move file into trash");
    }

    pub fn add_connection(&mut self, connection_tx: Sender<Message>) {
        self.connections.insert(self.next_id, connection_tx);
        self.next_id += 1;
    }

    pub fn new(library_path: &String) -> Vec<Media> {
        let mut media: Vec<Media> = vec![];

        rexiv2::set_log_level(rexiv2::LogLevel::MUTE);

        for entry in walkdir::WalkDir::new(library_path) {
            let path = entry.expect("Unable to get entry path");
            let path = path.path();
            if path.is_dir() {
                continue;
            }

            match mime_guess::from_path(&path).first() {
                Some(mime) if mime.type_() == "image" && mime.subtype() == "gif" => {
                    media.insert(
                        media.len(),
                        Media::new(path.display().to_string(), Category::Gif),
                    );
                }
                Some(mime) if mime.type_() == "image" => {
                    media.insert(
                        media.len(),
                        Media::new(path.display().to_string(), Category::Picture),
                    );
                }
                _ => {}
            };
        }

        media
    }
}
