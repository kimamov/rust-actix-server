use actix_multipart::{Field, Multipart};
use actix_web::web;
use futures::StreamExt;
/* use serde::{Deserialize, Serialize}; */
use std::io::Write;
use std::str;

use crate::models::Project;

#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub name: String,
    pub path: String,
}
impl UploadedFile {
    fn new(filename: &str) -> UploadedFile {
        UploadedFile {
            name: filename.to_string(),
            path: format!("./files/{}", filename),
        }
    }
}



pub async fn split_payload(payload: &mut Multipart) -> Project {
    let mut files: Vec<String> = Vec::new();

    /* fill with default values for now */
    let mut project: Project = Project {
        id: None,
        name: "".to_string(),
        description: "".to_string(),
        homepage: "".to_string(),
        repository: "".to_string(),
        packageLink: None,
        priority: None,
        images: None
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect(" split_payload err");
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();
        if name != "images" {
            /* println!("called outer loop"); */
            while let Some(chunk) = field.next().await {
                /* println!("called Inner"); */
                let data = chunk.expect("split_payload err chunk");
                /* convert bytes to string and print it  (just for testing) */

                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    /* println!("{:?}", data_string); */
                    /* all not file fields of your form (feel free to fix this mess) */
                    match name {
                        "title" => project.name = data_string,
                        "description" => project.description = data_string,
                        "homepage" => project.homepage= data_string,
                        "repository" => project.repository= data_string,
                        "priority" => project.repository = data_string.parse().expect("not a number"),
                        _=> println!("invalid field found")
                    };

                };
            }
        } else {
            match content_type.get_filename() {
                Some(filename) => {
                    let file = UploadedFile::new(filename); // create new UploadedFiles
                    let file_path = file.path.clone();
                    let mut f = web::block(move || std::fs::File::create(&file_path))
                        .await
                        .unwrap();  // create file at path
                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        f = web::block(move || f.write_all(&data).map(|_| f))
                            .await
                            .unwrap(); // write data chunks to file
                    }
                    files.push(file.name); // form only needs name
                }
                None => {
                    println!("file none");
                }
            }
        }
    }
    project.images=Some(files);
    project
}
