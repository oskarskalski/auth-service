use crate::schema::{teams, teammember};
use crate::team::models::team::Team;
use crate::team::models::team_member::TeamMember;
use diesel::pg::PgConnection;

pub fn select_team(conn: &mut PgConnection, team_id: String) -> Option<Team> {
    use diesel::prelude::*;
    teams::table
        .filter(teams::team_id.eq(&team_id))
        .get_result::<Team>(conn)
        .ok()
}

pub fn select_team_by_name(conn: &mut PgConnection, name: String) -> Option<Team> {
    use diesel::prelude::*;
    teams::table
        .filter(teams::team_name.eq(&name))
        .get_result::<Team>(conn)
        .ok()
}

pub fn select_team_member_by_team_member_id(
    conn: &mut PgConnection,
    team_member_id: String,
) -> Option<TeamMember> {
    use diesel::prelude::*;
    teammember::table
        .filter(teammember::team_member_id.eq(&team_member_id))
        .get_result::<TeamMember>(conn)
        .ok()
}

pub fn select_team_member_by_team_id(
    conn: &mut PgConnection,
    team_id: String,
) -> Option<TeamMember> {
    use diesel::prelude::*;
    teammember::table
        .filter(teammember::team_id.eq(&team_id))
        .get_result::<TeamMember>(conn)
        .ok()
}

pub fn select_team_id_by_user_id(conn: &mut PgConnection, user_id: String) -> Vec<TeamMember> {
    use diesel::prelude::*;
    let team_member = teammember::table
        .filter(teammember::user_id.eq(&user_id))
        .get_results::<TeamMember>(conn)
        .ok();
    let team_members = match team_member {
        Some(data) => data,
        None => vec![],
    };
    return team_members;
}

pub fn select_team_member(
    conn: &mut PgConnection,
    team_id: String,
    user_id: String,
) -> Option<TeamMember> {
    use diesel::prelude::*;
    teammember::table
        .filter(teammember::team_id.eq(&team_id))
        .filter(teammember::user_id.eq(&user_id))
        .get_result::<TeamMember>(conn)
        .ok()
}
