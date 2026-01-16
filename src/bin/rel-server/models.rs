use diesel::prelude::*;
use serde::Serialize;

#[derive(Identifiable, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::deploys)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Deploy {
    pub id: i32,
    pub environment_name: String,
    pub build_number: i32,
    pub version_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::deploys)]
pub struct NewDeploy {
    pub environment_name: String,
    pub build_number: i32,
    pub version_name: String,
}
