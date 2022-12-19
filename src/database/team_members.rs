use diesel::pg::PgConnection;

use diesel::{Insertable, Queryable, RunQueryDsl};

use crate::schema::teammember;

#[derive(Queryable, Insertable)]
#[table_name = "teammember"]
pub struct TeamMember {
    pub team_member_id: String,
    pub user_id: String,
    pub team_id: String,
    pub role_id: String,
}

pub fn insert_member(conn: &mut PgConnection, new_team_member: TeamMember) -> TeamMember {
    diesel::insert_into(teammember::table)
        .values(&new_team_member)
        .execute(conn)
        .unwrap();
    new_team_member
}

pub fn select_team_member_by_team_member_id(conn: &mut PgConnection, team_member_id: String) -> Option<TeamMember> {
    use diesel::prelude::*;
    teammember::table
        .filter(teammember::team_member_id.eq(&team_member_id))
        .get_result::<TeamMember>(conn)
        .ok()
}

pub fn select_team_member_by_team_id(conn: &mut PgConnection, team_id: String) -> Option<TeamMember> {
    use diesel::prelude::*;
    teammember::table
        .filter(teammember::team_id.eq(&team_id))
        .get_result::<TeamMember>(conn)
        .ok()
}

pub fn select_team_id_by_user_id(conn: &mut PgConnection, user_id: String) -> Option<String> {
    use diesel::prelude::*;
    let team_member = teammember::table
        .filter(teammember::user_id.eq(&user_id))
        .get_result::<TeamMember>(conn)
        .ok();
    let team_id = match team_member {
        Some(data) => Some(data.team_id),
        None => None
    };
    return team_id;
}