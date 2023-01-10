use diesel::pg::PgConnection;

use crate::{roles::models::roles::SystemRole, schema::systemrole};

pub fn select_roles(conn: &mut PgConnection) -> Vec<SystemRole> {
    use diesel::prelude::*;
    let result = systemrole::table
        .get_results::<SystemRole>(conn);
    return match result {
        Ok(data) => data,
        Err(errror) => panic!("Couldn't retrieve system roles")
    };
}
 