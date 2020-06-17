use crate::models::{Project, User, UserName};
use deadpool_postgres::Client;
/* use tokio_postgres; */
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_projects(
    client: &Client,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Project>, io::Error> {
    let statement = client
        .prepare("select * from projects order by priority desc limit $1 offset $2")
        .await
        .unwrap();

    let projects = client
        .query(&statement, &[&limit, &offset])
        .await
        .expect("Error getting projects")
        .iter()
        .map(|row| Project::from_row_ref(row).unwrap())
        .collect::<Vec<Project>>();

    Ok(projects)
}

pub async fn create_project(client: &Client, project: Project) -> Result<Project, io::Error> {
    let statement = client
        .prepare("insert into projects (name, description, homepage, repository, priority, images, technologies) values ($1, $2, $3, $4, $5, $6, $7) returning *")
        .await
        .unwrap();

    client
        .query(
            &statement,
            &[
                &project.name,
                &project.description,
                &project.homepage,
                &project.repository,
                &project.priority,
                &project.images,
                &project.technologies,
            ],
        )
        .await
        .expect("Error creating project")
        .iter()
        .map(|row| Project::from_row_ref(row).unwrap())
        .collect::<Vec<Project>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating project",
        ))
}

// add update project

pub async fn log_in(client: &Client, user_name: String) -> Result<User, io::Error> {
    let statement = client
        .prepare("select * from users where name = $1")
        .await
        .unwrap();

    client
        .query(&statement, &[&user_name])
        .await
        .expect("Error getting user")
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error getting user"))
}

pub async fn create_user(client: &Client, data: User) -> Result<UserName, io::Error> {
    let statement = client
        .prepare("insert into users (name, password) values ($1, $2) on conflict (name) do update set password = $2 returning name")
        .await
        .unwrap();

    client
        .query(&statement, &[&data.name, &data.password])
        .await
        .expect("Error getting user")
        .iter()
        .map(|row| UserName::from_row_ref(row).unwrap())
        .collect::<Vec<UserName>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating admin"))
}
