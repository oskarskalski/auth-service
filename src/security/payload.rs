use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};

use crate::{utils::errors::ValidationErrors, team::models::team_member::TeamMember};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    pub timestamp: u64,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_role: Option<i32>,
    pub expire_time: u64,
}

impl Payload {
    pub fn new(
            user_id: String,
            team_id: Option<String>,
            role_id: Option<i32>,
            group_id: Option<String>,
            group_role: Option<i32>
        ) -> Payload{
            let start: SystemTime = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let timestamp: u64 = since_the_epoch.as_secs();
            let expire_time: u64 = since_the_epoch.as_secs() + 86400;
            Payload {
                timestamp: timestamp,
                user_id: user_id.to_string(),
                expire_time: expire_time,
                team_id: team_id,
                role_id: role_id,
                group_id: group_id,
                group_role: group_role,
            }
        }
        
    pub fn create(user_id: String, team_id: Option<String>, group_id: Option<String>) -> Result<Payload, ValidationErrors> {
        let role_id = match team_id.clone() {
            Some(id) => match TeamMember::find_team_member(user_id.clone(), id) {
                    Ok(data) => Some(data.role_id),
                    Err(err) => return Err(err)
            },
            None => None
        };
        let payload = Payload::new(user_id, team_id, role_id, group_id, None);
        return Ok(payload);
    }
}