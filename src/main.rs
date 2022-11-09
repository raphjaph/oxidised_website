use {
    serde_derive::Deserialize,
    std::net::SocketAddr,
    std::process::Command,
    warp::{fs, get, path, Filter},
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

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = read_config();

    build_site();

    let address: SocketAddr = "127.0.0.1:3030".parse().expect("Unable to parse address");

    let zola_site = get().and(fs::dir(config.zola_site));

    let favicon = get()
        .and(path("favicon.ico"))
        .and(fs::file("./public/images/favicon.png"));

    let static_assets = path("s").and(fs::dir(config.static_dir));

    let routes = zola_site.or(static_assets).or(favicon);

    warp::serve(routes).run(address).await;
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
