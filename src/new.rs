// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;

use owo_colors::*;

#[allow(warnings)]
use sea_orm::QueryResult;

/// TODO: add docs here.
#[derive(Clone, RustEmbed, Debug)]
#[folder = "./migrations"]
pub struct NewMigrations;

// Runs database's migrations.
pub async fn run_migrations() -> Result<()> {
    let db_conn = DATABASES_CONNS
        .get()
        .unwrap()
        .search(None)?
        .to_owned()
        .into_arc_any()
        .downcast::<mysql::MySQLConnection>()
        .unwrap();

    let new_project_script = String::from_utf8(
        NewMigrations::get("new_project.sql")
            .unwrap()
            .data()
            .to_vec(),
    )?; // it requires `.to_vec()` at release.

    let queries = new_project_script
        .lines()
        .map(|line| line.to_compact_string())
        .filter(|line| !line.starts_with("--"))
        .collect::<String>();

    #[allow(warnings)]
    let mut statements = queries
        .split(";")
        .map(|query| query.to_compact_string())
        .collect::<CheapVec<CompactString>>();

    // FIX: sea-schema won't accept MariaDB's default database's encoding on Windows...
    #[cfg(target_os = "windows")]
    {
        debug!("Changing database's default encoding.");

        db_conn
            .execute(databases::DatabaseInput::Query(
                "SET FOREIGN_KEY_CHECKS=0".into(),
            ))
            .await?;
        db_conn
            .execute(databases::DatabaseInput::Query(
                "SET SESSION character_set_server = 'utf8mb4'".into(),
            ))
            .await?;
        db_conn
            .execute(databases::DatabaseInput::Query(
                "SET SESSION collation_server = 'utf8mb4_unicode_ci'".into(),
            ))
            .await?;

        // Change the encoding for new tables.
        statements.insert_many(
            0,
            [format!(
                "ALTER DATABASE {} CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;",
                db_name
            )
            .into()],
        );

        statements.push("SET FOREIGN_KEY_CHECKS=1".into());

        // Convert current tables.
        let DatabaseOutput::Any(execute) = db_conn
            .execute(databases::DatabaseInput::Query(
                format!(
                    "SELECT CONCAT(
                        'ALTER TABLE `', TABLE_SCHEMA, '`.`', TABLE_NAME, '` ',
                        'CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci, ',
                        'DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;'
                     ) AS execute
                     FROM information_schema.TABLES
                     WHERE TABLE_SCHEMA = '{}' AND TABLE_TYPE = 'BASE TABLE'",
                    db_name
                )
                .into(),
            ))
            .await?
        else {
            bail!("Couldn't change the encoding of existing database's tables.");
        };

        let res = execute
            .downcast::<Vec<QueryResult>>()
            .map_err(|err| anyhow!("Cannot downcast to MySQL query result. {:?}", err))?;

        for entry in *res {
            if let Ok(query) = entry.try_get::<String>("", "execute") {
                db_conn.execute(DatabaseInput::Query(query.into())).await?;
            }
        }
    }

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
    db_user: Option<CompactString>,
    db_password: Option<CompactString>,
) -> Result<CompactString> {
    // Create a new Silence project.
    let mut config = config::Config::default();

    // Execute database's migrations if database data is given.
    if let (Some(db_name), Some(db_user), Some(db_password)) = (db_name, db_user, db_password) {
        *config.database_conn_mut() = databases::mysql::MySQLDBConnectionConfig::new(
            db_host.unwrap_or(SocketAddr::new("127.0.0.1".parse().unwrap(), 3306)),
            db_user,
            db_password,
            db_name.to_owned(),
        );

        DatabasesConnections::load(CheapVec::from_vec(vec![config.into_database_config()])).await?;

        run_migrations().await?;
    } else {
        *config.database_conn_mut() = databases::mysql::MySQLDBConnectionConfig::new(
            db_host.unwrap_or(SocketAddr::new("127.0.0.1".parse().unwrap(), 3306)),
            "db_user".into(),
            "db_password".into(),
            "db_password".into(),
        );

        println!("{}\n{}", "Database connection config were not passed as arguments, using default values instead.".bright_yellow(), " Migrations were not run, modify database connection config in `config.json` and run migrations.".bright_red().blink());
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
