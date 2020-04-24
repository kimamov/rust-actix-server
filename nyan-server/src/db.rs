use crate::models::{Project, User, UserName};
use deadpool_postgres::Client;
use tokio_postgres;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_projects(client: &Client) -> Result<Vec<Project>, io::Error> {
    let statement = client.prepare("select * from projects").await.unwrap();

    let projects = client
        .query(&statement, &[])
        .await
        .expect("Error getting projects")
        .iter()
        .map(|row| Project::from_row_ref(row).unwrap())
        .collect::<Vec<Project>>();

    Ok(projects)
}

pub async fn create_project(client: &Client, project: Project)->Result<Project, io::Error> {
    let statement = client
        .prepare("insert into projects (name, description, homepage, repository, priority, images) values ($1, $2, $3, $4, $5, $6) returning *")
        .await
        .unwrap();

    client
        .query(&statement, &[&project.name, &project.description, &project.homepage, &project.repository, &project.priority, &project.images])
        .await
        .expect("Error creating project")
        .iter()
        .map(|row| Project::from_row_ref(row).unwrap())
        .collect::<Vec<Project>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating project"))
}


pub async fn log_in(client: &Client, data: User)->  Result<UserName, io::Error>{
    let statement = client.prepare("select name from users where name = $1 and password = $2").await.unwrap();

    client
        .query(&statement, &[&data.name, &data.password])
        .await
        .expect("Error getting user")
        .iter()
        .map(|row| UserName::from_row_ref(row).unwrap())
        .collect::<Vec<UserName>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error getting user"))

    
}