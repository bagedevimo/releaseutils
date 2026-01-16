use crate::{PooledSqliteConnection, SqliteConnectionPool};

use super::handlers;
use super::model::DeployParameters;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use gitlab::AsyncGitlab;
use warp::Filter;

fn with_deploy_parameters(
) -> impl Filter<Extract = (DeployParameters,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_gitlab(
    gitlab: AsyncGitlab,
) -> impl Filter<Extract = (AsyncGitlab,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || gitlab.clone())
}

fn with_database(
    pool: SqliteConnectionPool,
) -> impl Filter<Extract = (PooledSqliteConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.get().unwrap())
}

pub fn releaseutils(
    gitlab: AsyncGitlab,
    db: Pool<ConnectionManager<SqliteConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list_deploys(db.clone()).or(upsert_deploy(db.clone(), gitlab.clone()))
}

pub fn list_deploys(
    db: SqliteConnectionPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "deploys")
        .and(warp::get())
        .and(with_database(db))
        .and_then(handlers::list_deploys)
}

pub fn upsert_deploy(
    db: SqliteConnectionPool,
    gitlab: AsyncGitlab,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "deploys" / String)
        .and(warp::post())
        .and(with_database(db))
        .and(with_gitlab(gitlab))
        .and(with_deploy_parameters())
        .and_then(handlers::upsert_deploy)
}
