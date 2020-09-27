#[macro_use]
extern crate log;
use bollard::Docker;
use config::DockerConnectMode;

mod config;
mod container;
mod steam;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting gameserver updater");

    let config = config::get_config()?;

    let mut containers = config.containers;
    let interval = config.check_interval;
    let key = config.steam_api_key;

    let docker_client = match config.connect_mode {
        Some(DockerConnectMode::UnixSocket) => Docker::connect_with_unix_defaults()?,
        Some(DockerConnectMode::Http) => Docker::connect_with_http_defaults()?,
        Some(DockerConnectMode::SSL) => Docker::connect_with_ssl_defaults()?,
        None => return Err("Connection mode is None, this shouldn't happen".into()),
    };

    for container in containers.iter_mut() {
        container.init(&key);
    }
    info!("Sleeping for {}s", interval.as_secs());
    std::thread::sleep(interval);

    loop {
        for container in containers.iter_mut() {
            container.update(&key, &docker_client);
        }
        info!("Sleeping for {}s", interval.as_secs());
        std::thread::sleep(interval);
    }
}
