// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;

#[allow(warnings)]
use sea_orm::QueryResult;

/// TODO: add docs here.
#[derive(Clone, RustEmbed, Debug)]
#[folder = "./migrations"]
pub struct NewMigrations;

// Runs database's migrations.
pub async fn run_migrations(_internal_db_name: Option<CompactString>) -> Result<()> {
    let db_conns = DATABASES_CONNS.get().unwrap().to_owned();

    let _main_db_conn = db_conns
        .search(None)?
        .into_arc_any()
        .downcast::<mysql::MySQLConnection>()
        .unwrap();

    let db_conn = match _internal_db_name {
        Some(_) => db_conns
            .search(Some("internal".into()))?
            .into_arc_any()
            .downcast::<mysql::MySQLConnection>()
            .unwrap(),
        None => _main_db_conn,
    };

    let internal_script =
        String::from_utf8(NewMigrations::get("internal.sql").unwrap().data().to_vec())?; // it requires `.to_vec()` at release.

    let queries = internal_script
        .lines()
        .map(|line| line.to_compact_string())
        .filter(|line| !line.starts_with("--"))
        .collect::<String>();

    #[allow(warnings)]
    let mut statements = queries
        .split(";")
        .map(|query| query.to_compact_string())
        .collect::<CheapVec<CompactString>>();

    let mut statements = statements.into_iter();

    while let Some(statement) = statements.next() {
        if !statement.is_empty() {
            db_conn
                .execute(databases::DatabaseInput::Query(statement.into()))
                .await?;
        }
    }

    Ok(())
}

/// Create a new project in the current dir with the specified settings.
pub async fn new_project(
    name: CompactString,
    db_host: Option<SocketAddr>,
    db_name: Option<CompactString>,
    internal_db_name: Option<CompactString>,
    db_user: Option<CompactString>,
    db_password: Option<CompactString>,
    prefer_web_directory: bool,
) -> Result<CompactString> {
    // Create a new Silence project.
    let mut config = config::Config::default();

    // Execute database's migrations if database data is given.
    if let (Some(db_name), Some(db_user), Some(db_password)) = (db_name, db_user, db_password) {
        *config.databases_conn_mut() = config::DatabasesConnectionConfig::new(
            mysql::MySQLDBConnectionConfig::new(
                db_host.unwrap_or(SocketAddr::new("127.0.0.1".parse().unwrap(), 3306)),
                db_user.to_owned(),
                db_password.to_owned(),
                db_name.to_owned(),
            ),
            match &internal_db_name {
                Some(internal_db_name) => Some(mysql::MySQLDBConnectionConfig::new(
                    db_host.unwrap_or(SocketAddr::new("127.0.0.1".parse().unwrap(), 3306)),
                    db_user,
                    db_password,
                    internal_db_name.to_owned(),
                )),
                None => None,
            },
        );

        DatabasesConnections::load(config.into_database_config()).await?;

        run_migrations(internal_db_name).await?;
    }

    // Create the project's folder.
    let project_path = current_dir()?.join(&name);

    // Check whether a folder with the project's name already exists.
    if try_exists(&project_path).await? {
        return Err(anyhow!("A folder with the project's name already exists."));
    }

    if let Err(err) = create_dir(&project_path).await {
        Err(anyhow!(
            "Cannot create project's folder {}. Are you sure that there is no project with the same name and that you have write permissions?%{}",
            name,
            err.to_string()
        ))?;
    }

    // Write the `config.json` file.
    let project_file = project_path.join("config.json");

    let content = serde_json::to_string_pretty(&config)?;

    write(&project_file, content).await?;

    // Generate all project's subfolders.
    {
        create_dir(project_path.join("endpoints")).await?;

        create_dir(project_path.join(if prefer_web_directory {
            "web"
        } else {
            "static"
        }))
        .await?;
    }

    return Ok(format!(
        "Silence project `{}` was created at `{}`.",
        name,
        project_path.display()
    )
    .to_compact_string());
}
