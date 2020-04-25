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
    pub packageLink: Option<String>,
    pub priority: Option<i16>,
    pub images: Option<Vec<String>>,
}


#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub password: Option<String>
}

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct UserName {
    pub name: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LogInMessage {
    pub succes: bool,
    pub message: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SearchParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>
}

impl Default for SearchParams {
    fn default()->SearchParams {
        SearchParams {
            limit: None,
            offset: None
        }
    }
}