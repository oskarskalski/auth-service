use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};

use diesel::{Insertable, Queryable, RunQueryDsl};

use crate::{
    database::team_members::TeamMember,
    schema::{teammember, teamrole, teamrolemap, teamroles},
};

#[derive(Serialize, Deserialize, Debug, Queryable, Insertable)]
#[table_name = "teamrole"]
pub struct TeamRole {
    pub team_role_id: String,
    pub role_name: String,
    pub team_id: String,
}

#[derive(Serialize, Deserialize, Debug, Queryable, Insertable)]
#[table_name = "teamroles"]
pub struct TeamRoles {
    pub id: i32,
    pub role_name: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "teamrolemap"]
pub struct TeamRoleMap {
    pub id: Option<i32>,
    pub role_id: i32,
    pub team_role_id: String,
}

pub fn insert_team_role_map(conn: &mut PgConnection, team_role_map: TeamRoleMap) -> TeamRoleMap {
    diesel::insert_into(teamrolemap::table)
        .values(&team_role_map)
        .execute(conn)
        .unwrap();
    team_role_map
}

pub fn insert_team_role(conn: &mut PgConnection, new_team: TeamRole) -> String {
    let result = diesel::insert_into(teamrole::table)
        .values(&new_team)
        .returning(teamrole::team_role_id)
        .get_result::<String>(conn);
    let result = match result {
        Ok(t) => t,
        Err(err) => String::from(""),
    };
    return result;
}

pub fn select_role_ids(conn: &mut PgConnection) -> Vec<i32> {
    use diesel::prelude::*;
    let result = teamroles::table
        .select(teamroles::id)
        .get_results::<i32>(conn)
        .ok();
    let result = match result {
        Some(data) => data,
        None => vec![],
    };
    return result;
}

pub fn select_roles(conn: &mut PgConnection) -> Vec<TeamRoles> {
    use diesel::prelude::*;
    let result = teamroles::table.get_results::<TeamRoles>(conn).ok();
    let result = match result {
        Some(data) => data,
        None => vec![],
    };
    return result;
}

pub fn select_team_roles(conn: &mut PgConnection, team_id: String) -> Vec<TeamRole> {
    use diesel::prelude::*;
    let result = teamrole::table
        .filter(teamrole::team_id.eq(&team_id))
        .load::<TeamRole>(conn);
    return match result {
        Ok(roles) => roles,
        Err(e) => vec![],
    };
}

pub fn save_role_relation(conn: &mut PgConnection, team_role_map: TeamRoleMap) {
    diesel::insert_into(teamrolemap::table)
        .values(&team_role_map)
        .execute(conn)
        .unwrap();
}

pub fn select_team_member_role(
    conn: &mut PgConnection,
    team_id: String,
    user_id: String,
) -> Vec<TeamRole> {
    use diesel::prelude::*;
    let result = teamrole::table
        .filter(teamrole::team_id.eq(&team_id))
        .load::<TeamRole>(conn);
    return match result {
        Ok(roles) => roles,
        Err(e) => vec![],
    };
}
pub fn select_team_member(conn: &mut PgConnection, team_id: String, user_id: String) {
    use diesel::prelude::*;
    let result = teammember::table
        .filter(teammember::team_id.eq(&team_id))
        .filter(teammember::user_id.eq(&team_id))
        .load::<TeamMember>(conn);
}
