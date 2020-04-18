use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status{
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="projects")]
pub struct Project{
    pub id: i32,
    pub name: String,
    gif_path: String,
    description: String,
    homepage: String,
    repository: String,
    image_paths: Option<Vec<String>>
}