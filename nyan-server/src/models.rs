use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "projects")]
pub struct Project {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub repository: String,
    pub priority: Option<i16>,
    pub images: Option<Vec<String>>,
}

/* #[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    title: String,
    description: String,
    homepage: String,
    repository: String,
    priority: u32,
    images: Vec<String>,
} */