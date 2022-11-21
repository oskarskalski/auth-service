// @generated automatically by Diesel CLI.

diesel::table! {
    teams (team_id) {
        team_id -> Varchar,
        creator_id -> Varchar,
        team_name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Varchar,
        username -> Varchar,
        e_mail -> Varchar,
        password -> Varchar,
        created_at -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    teams,
    users,
);
