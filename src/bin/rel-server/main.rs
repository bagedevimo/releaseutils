#![deny(warnings)]

mod errors;
mod filters;
mod handlers;
mod models;
mod schema;

use diesel::migration::MigrationVersion;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use gitlab::{AsyncGitlab, GitlabBuilder};
use log::info;
use r2d2::PooledConnection;
use std::env;
use std::error::Error;
use warp::Filter;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use errors::{RelError, Result};
use releaseutils::logging::setup_logging;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    setup_logging();

    let gitlab = setup_gitlab_client().await?;
    let db = setup_db().await?;

    let api = filters::releaseutils(gitlab, db);
    let routes = api.with(warp::log("rel_server"));

    info!("Serving on 127.0.0.1:3030");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

async fn setup_gitlab_client() -> Result<AsyncGitlab> {
    let token =
        std::env::var("GITLAB_PRIVATE_TOKEN").or_else(|_| Err(RelError::MissingGitlabToken))?;

    let builder = GitlabBuilder::new("gitlab.com", token);

    match builder.build_async().await {
        Ok(gl) => Ok(gl),
        Err(e) => Err(RelError::GitlabError(e)),
    }
}

async fn setup_db() -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    let url = env::var("DATABASE_URL").or_else(|_| Err(RelError::MissingDatabaseUrl))?;
    let manager = ConnectionManager::<SqliteConnection>::new(url);

    match Pool::builder().test_on_check_out(true).build(manager) {
        Ok(pool) => {
            let _ = run_migrations(&mut pool.get().unwrap());
            Ok(pool)
        }
        Err(e) => Err(RelError::DbPoolError(e)),
    }
}

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> std::result::Result<Vec<MigrationVersion<'_>>, Box<dyn Error + Send + Sync + 'static>> {
    match connection.has_pending_migration(MIGRATIONS) {
        Ok(true) => {
            info!("Database out of date, migrating..");
            return connection.run_pending_migrations(MIGRATIONS);
        }
        _ => Ok(vec![]),
    }
}

mod model {
    use serde::Serialize;
    use serde_derive::Deserialize;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeployParameters {
        pub sha: String,
        pub finalised: Option<bool>,
        pub version_name: String,
        pub build_number: i32,
    }
}

type SqliteConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
type PooledSqliteConnection = PooledConnection<ConnectionManager<SqliteConnection>>;
