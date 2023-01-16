use serde::{Deserialize, Serialize};
use std::io::Error as IoError;
use std::{fs, vec};
use toml;

const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

pub const TOKEN_PREFIX: &'static str = "Bearer ";

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    database: Option<ConfigTomlDatabase>,
    jwt: Option<ConfigTomlJwt>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDatabase {
    hostname: Option<String>,
    name: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlJwt {
    secret_token: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub hostname: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub secret_token: String,
}

impl Config {
    pub fn new(external_config: Option<&str>) -> Config {
        let mut config_filepaths: Vec<&str> = vec!["./Config.toml"];

        match external_config {
            Some(config) => config_filepaths.push(config),
            None => (),
        };

        let mut content: String = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml {
                database: None,
                jwt: None,
            }
        });

        let (hostname, name, username, password): (String, String, String, String) =
            match config_toml.database {
                Some(database) => {
                    let db_hostname: String = database.hostname.unwrap_or_else(|| {
                        println!("Missing field username in table database.");
                        "unknown".to_owned()
                    });

                    let db_name: String = database.name.unwrap_or_else(|| {
                        println!("Missing field username in table database.");
                        "unknown".to_owned()
                    });

                    let db_username: String = database.username.unwrap_or_else(|| {
                        println!("Missing field username in table database.");
                        "unknown".to_owned()
                    });

                    let db_password: String = database.password.unwrap_or_else(|| {
                        println!("Missing field password in table database.");
                        "unknown".to_owned()
                    });

                    (db_hostname, db_name, db_username, db_password)
                }
                None => panic!("Missing database information from the config"),
            };

        let secret_token: String = match config_toml.jwt {
            Some(jwt) => jwt
                .secret_token
                .unwrap_or_else(|| panic!("Missing JWT secret")),
            None => panic!("Missing JWT secret"),
        };

        Config {
            username: username,
            password: password,
            secret_token: secret_token,
            hostname,
            name,
        }
    }
}
