use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::{schema::{systemrole, roles, rolesmap}, utils::db_connection::establish_connection};
use crate::roles::database::select::select_roles;

#[derive(Serialize, Deserialize, Debug, Queryable, Insertable)]
#[table_name = "systemrole"]
pub struct SystemRole {
    pub role_id: i32,
    pub role_name: String,
    pub role_type: String,
    pub role_description: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Queryable, Insertable)]
#[table_name = "roles"]
pub struct Roles {
    pub id: i32,
    pub role_name: String,
    pub role_description: Option<String>,
}

#[derive(Queryable, Insertable)]
#[table_name = "rolesmap"]
pub struct TeamRoleMap {
    pub id: Option<i32>,
    pub role_id: i32,
    pub roles_id: i32,
}
use rocket::{self, State};


impl SystemRole {
    pub fn get_all() -> Vec<SystemRole> {
        let connection = &mut establish_connection();
        select_roles(connection)
    }

    pub fn get_team_role(user_id: String, team_id: String) -> i32 {
        let connection = &mut establish_connection();
        return 1;
    }
}
