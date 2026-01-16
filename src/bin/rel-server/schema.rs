// @generated automatically by Diesel CLI.

diesel::table! {
    deploy_events (id) {
        id -> Integer,
        deploy_id -> Integer,
    }
}

diesel::table! {
    deploys (id) {
        id -> Integer,
        environment_name -> Text,
        build_number -> Integer,
        version_name -> Text,
    }
}

diesel::joinable!(deploy_events -> deploys (deploy_id));

diesel::allow_tables_to_appear_in_same_query!(deploy_events, deploys,);
