#[macro_use]
extern crate rocket;

use {
    rocket::fs::FileServer,
    serde_derive::Deserialize,
    std::process::Command,
};

#[derive(Deserialize)]
struct TomlFile {
    extra: Config,
}

#[derive(Deserialize)]
struct Config {
    zola_site: String,
    static_dir: String,
}

#[launch]
fn rocket() -> _ {
    build_site();
    let config = parse_config();

    rocket::build()
        .mount("/", FileServer::from(config.zola_site).rank(1))
        .mount("/s", FileServer::from(config.static_dir))
}

fn parse_config() -> Config {
    let contents = std::fs::read_to_string("config.toml").expect("could not read config.toml");
    let config: TomlFile = toml::from_str(&contents).expect("unable to parse config");

    config.extra
}

fn build_site() {
    Command::new("zola")
        .arg("build")
        .status()
        .expect("could not build zola site");
}
