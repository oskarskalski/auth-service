use diesel::pg::PgConnection;

use diesel::{Insertable, Queryable, RunQueryDsl};

use crate::schema::teams;

#[derive(Queryable, Insertable)]
#[diesel(table_name = teams)]
pub struct Team {
    pub team_id: String,
    pub team_name: String,
    pub description: String,
}

pub fn insert_team(conn: &mut PgConnection, new_team: Team) -> Team {
    diesel::insert_into(teams::table)
        .values(&new_team)
        .execute(conn)
        .unwrap();
    new_team
}

pub fn select_team(conn: &mut PgConnection, team_id: String) -> Option<Team> {
    use diesel::prelude::*;
    teams::table
        .filter(teams::team_id.eq(&team_id))
        .get_result::<Team>(conn)
        .ok()
}
