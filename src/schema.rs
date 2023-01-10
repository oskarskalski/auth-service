// @generated automatically by Diesel CLI.

diesel::table! {
    groups (group_id) {
        group_id -> Varchar,
        team_id -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    groupuser (user_id) {
        user_id -> Varchar,
        group_id -> Varchar,
        role_id -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        role_name -> Varchar,
        role_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    rolesmap (id) {
        id -> Int4,
        role_id -> Int4,
        roles_id -> Int4,
    }
}

diesel::table! {
    systemrole (role_id) {
        role_id -> Int4,
        role_name -> Varchar,
        role_type -> Varchar,
        role_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    teammember (team_member_id) {
        team_member_id -> Varchar,
        user_id -> Varchar,
        team_id -> Varchar,
        role_id -> Int4,
    }
}

diesel::table! {
    teams (team_id) {
        team_id -> Varchar,
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

diesel::joinable!(groups -> teams (team_id));
diesel::joinable!(groupuser -> groups (group_id));
diesel::joinable!(groupuser -> systemrole (role_id));
diesel::joinable!(groupuser -> users (user_id));
diesel::joinable!(rolesmap -> roles (roles_id));
diesel::joinable!(rolesmap -> systemrole (role_id));
diesel::joinable!(teammember -> systemrole (role_id));
diesel::joinable!(teammember -> teams (team_id));
diesel::joinable!(teammember -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    groupuser,
    roles,
    rolesmap,
    systemrole,
    teammember,
    teams,
    users,
);
