use std::collections::HashMap;

pub struct Context {
    pub tidb_service: TiDBService,
}

pub struct TiDBService {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database: String,

    pub label_table_map: HashMap<String, String>,
    pub table_id_map: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            tidb_service: TiDBService {
                host: String::from("localhost"),
                port: String::from("4000"),
                username: String::from("root"),
                password: String::from(""),
                database: String::from("test"),

                label_table_map: HashMap::new(),
                table_id_map: HashMap::new(),
            },
        }
    }
}
