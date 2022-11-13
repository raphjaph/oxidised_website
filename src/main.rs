#[macro_use]
extern crate rocket;

use {rocket::fs::FileServer, serde_derive::Deserialize, std::process::Command};

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

    // TODO: combine these two with figment <https://docs.rs/figment/0.10.8/figment/>
    let config = parse_config();
    let mut c = rocket::Config::default();
    c.address = "0.0.0.0".parse().expect("should parse ip address");
    c.port = 8080;

    rocket::custom(c)
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
