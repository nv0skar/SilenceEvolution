// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;

/// TODO: add docs here.
#[derive(Clone, RustEmbed, Debug)]
#[folder = "./migrations"]
pub struct NewMigrations;

/// Create a new project in the current dir with the specified settings
pub async fn new_project(
    name: CompactString,
    db_host: Option<SocketAddr>,
    db_name: CompactString,
    db_user: CompactString,
    db_password: CompactString,
) -> Result<CompactString> {
    // Create a new Silence project.
    let mut config = config::Config::default();

    *config.database_conn_mut() = databases::mysql::MySQLDBConnectionConfig::new(
        db_host.unwrap_or(SocketAddr::new("127.0.0.1".parse().unwrap(), 3306)),
        db_user,
        db_password,
        db_name,
    );

    // Execute database's migrations.
    DatabasesConnections::load(CheapVec::from_vec(vec![config.into_database_config()])).await?;

    let db_conn = DATABASES_CONNS
        .get()
        .unwrap()
        .search(None)?
        .to_owned()
        .into_arc_any()
        .downcast::<mysql::MySQLConnection>()
        .unwrap();

    let new_project_script =
        String::from_utf8(NewMigrations::get("new_project.sql").unwrap().data())?;

    let queries = new_project_script
        .lines()
        .map(|line| line.to_compact_string())
        .filter(|line| !line.starts_with("--"))
        .collect::<String>();

    let mut statements = queries.split(";");

    while let Some(statement) = statements.next() {
        if !statement.is_empty() {
            db_conn
                .execute(databases::DatabaseInput::Query(
                    statement.to_compact_string(),
                ))
                .await?;
        }
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

        create_dir(project_path.join("static")).await?;
    }

    return Ok(format!(
        "Silence project `{}` was created at `{}`.",
        name,
        project_path.display()
    )
    .to_compact_string());
}
