use crate::models::{Project};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_projects(client: &Client)->Result<Vec<Project>, io::Error>{
    let statement=client.prepare("select * from projects").await.unwrap();

    let projects=client.query(&statement, &[])
        .await.expect("Error getting projects")
        .iter()
        .map(|row| Project::from_row_ref(row).unwrap())
        .collect::<Vec<Project>>();

    Ok(projects)
}