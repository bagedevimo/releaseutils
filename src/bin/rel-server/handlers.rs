use crate::{
    model::DeployParameters,
    models::{Deploy, NewDeploy},
    PooledSqliteConnection,
};
use diesel::{RunQueryDsl, SelectableHelper};
use gitlab::AsyncGitlab;
use serde::Serialize;
use std::convert::Infallible;

pub async fn upsert_deploy(
    environment: String,
    mut db: PooledSqliteConnection,
    _gitlab: AsyncGitlab,
    params: DeployParameters,
) -> Result<impl warp::Reply, Infallible> {
    use crate::schema::deploys::dsl::*;

    let new_deploy = NewDeploy {
        environment_name: environment,
        build_number: params.build_number,
        version_name: params.version_name.clone(),
    };

    match diesel::insert_into(deploys)
        .values(&new_deploy)
        .returning(Deploy::as_returning())
        .get_result(&mut db)
    {
        Ok(deploy) => Ok(warp::reply::with_status(
            warp::reply::json(&deploy),
            warp::http::StatusCode::CREATED,
        )),
        Err(_) => Ok(warp::reply::with_status(
            warp::reply::json(&[1, 2, 3]),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn list_deploys(mut db: PooledSqliteConnection) -> Result<impl warp::Reply, Infallible> {
    use crate::schema::deploys::dsl::*;

    #[derive(Serialize)]
    struct ListDeploysResponse {
        deploys: Vec<crate::models::Deploy>,
    }

    let results = match deploys.load::<crate::models::Deploy>(&mut db) {
        Ok(r) => r,
        _ => Vec::new(),
    };

    let resp = ListDeploysResponse { deploys: results };

    Ok(warp::reply::json(&resp))
}
