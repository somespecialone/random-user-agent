use serde::Deserialize;

fn def_port() -> u16 {
    8080
}

fn def_deta_space_app() -> bool {
    false
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "def_deta_space_app")]
    pub deta_space_app: bool,
    #[serde(default = "def_port")]
    pub port: u16,
}
