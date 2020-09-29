#[macro_use]
extern crate log;
use bollard::Docker;
use config::{Config, DockerConnectMode};

mod config;
mod container;
mod steam;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting heat exchanger");

    let (mut containers, key, interval, state_dir, docker_connect_mode) = Config::get()?.consume();

    // Get a docker client based on the connection mode set
    // If the state storage directory doesn't exist, warn and create it
    if !state_dir.exists() {
        warn!(
            "State directory {} doesn't exist, creating...",
            &state_dir.display()
        );
        std::fs::create_dir(&state_dir)?;
    }

    let docker_client = match docker_connect_mode {
        DockerConnectMode::UnixSocket => Docker::connect_with_unix_defaults(),
        DockerConnectMode::Http => Docker::connect_with_http_defaults(),
        DockerConnectMode::SSL => Docker::connect_with_ssl_defaults(),
    }?;

    // Initialise all our containers
    for container in containers.iter_mut() {
        container.init(&key, &docker_client, &state_dir);
        container.save_state(&state_dir);
    }
    info!(
        "Startup complete, sleeping for {} seconds",
        interval.as_secs()
    );
    std::thread::sleep(interval);

    // Main program loop
    loop {
        for container in containers.iter_mut() {
            container.update(&key, &docker_client);
            container.save_state(&state_dir);
        }
        info!("Sleeping for {} seconds", interval.as_secs());
        std::thread::sleep(interval);
    }
}
