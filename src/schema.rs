// @generated automatically by Diesel CLI.

diesel::table! {
    grouprole (group_role_id) {
        group_role_id -> Varchar,
        group_id -> Varchar,
        role_name -> Varchar,
    }
}

diesel::table! {
    grouprolemap (id) {
        id -> Int4,
        group_role_id -> Varchar,
        group_roles_id -> Int4,
    }
}

diesel::table! {
    grouproles (id) {
        id -> Int4,
        role_name -> Varchar,
    }
}

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
        group_role_id -> Varchar,
    }
}

diesel::table! {
    teammember (team_member_id) {
        team_member_id -> Varchar,
        user_id -> Varchar,
        team_id -> Varchar,
        role_id -> Varchar,
    }
}

diesel::table! {
    teamrole (team_role_id) {
        team_role_id -> Varchar,
        role_name -> Varchar,
        team_id -> Varchar,
    }
}

diesel::table! {
    teamrolemap (id) {
        id -> Int4,
        role_id -> Int4,
        team_role_id -> Varchar,
    }
}

diesel::table! {
    teamroles (id) {
        id -> Int4,
        role_name -> Varchar,
    }
}

diesel::table! {
    teamroles2 (id) {
        id -> Int4,
        role_name -> Varchar,
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

diesel::joinable!(grouprole -> groups (group_id));
diesel::joinable!(grouprolemap -> grouprole (group_role_id));
diesel::joinable!(grouprolemap -> grouproles (group_roles_id));
diesel::joinable!(groups -> teams (team_id));
diesel::joinable!(groupuser -> grouprole (group_role_id));
diesel::joinable!(groupuser -> groups (group_id));
diesel::joinable!(groupuser -> users (user_id));
diesel::joinable!(teammember -> teamrole (role_id));
diesel::joinable!(teammember -> teams (team_id));
diesel::joinable!(teammember -> users (user_id));
diesel::joinable!(teamrole -> teams (team_id));
diesel::joinable!(teamrolemap -> teamrole (team_role_id));
diesel::joinable!(teamrolemap -> teamroles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    grouprole,
    grouprolemap,
    grouproles,
    groups,
    groupuser,
    teammember,
    teamrole,
    teamrolemap,
    teamroles,
    teamroles2,
    teams,
    users,
);
