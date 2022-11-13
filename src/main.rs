#[macro_use]
extern crate rocket;

use {
    rocket::fs::FileServer,
    serde_derive::Deserialize,
    std::process::Command,
};

#[derive(Deserialize)]
struct Config {
    extra: Extra,
}

#[derive(Deserialize)]
struct Extra {
    zola_site: String,
    static_dir: String,
}

#[launch]
fn rocket() -> _ {
    build_site();
    let config = read_config();

    let zola = FileServer::from(config.zola_site).rank(1);
    let heavy_content = FileServer::from(config.static_dir);

    rocket::build()
        .mount("/", zola)
        .mount("/s", heavy_content)
}

fn read_config() -> Extra {
    let contents = std::fs::read_to_string("config.toml").expect("could not read config.toml");
    let config: Config = toml::from_str(&contents).expect("unable to parse config");

    config.extra
}

fn build_site() {
    Command::new("zola")
        .arg("build")
        .status()
        .expect("could not build zola site");
}
