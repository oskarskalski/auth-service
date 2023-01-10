use diesel::pg::PgConnection;

use diesel::RunQueryDsl;
use log::{log, Level};

use crate::schema::{teams, teammember};
use crate::team::models::team::Team;
use crate::team::models::team_member::TeamMember;
use crate::utils::errors::DatabaseErrors;


pub fn insert_team(conn: &mut PgConnection, new_team: Team) -> Team {
    diesel::insert_into(teams::table)
        .values(&new_team)
        .execute(conn)
        .unwrap();
    new_team
}

pub fn insert_member(
    connection: &mut PgConnection,
    team_member: TeamMember,
) -> Result<TeamMember, DatabaseErrors> {
    use diesel::prelude::*;
    let result = diesel::insert_into(teammember::table)
        .values(&team_member)
        .execute(connection);
    return match result {
        Ok(_) => Ok(team_member),
        Err(e) => {
            log!(Level::Error, "Received errors: {}", e);
            return Err(DatabaseErrors::CannotInsertData(
                "Couldn't insert data".to_string(),
            ));
        }
    };
}
