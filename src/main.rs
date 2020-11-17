#[macro_use]
extern crate log;
use bollard::Docker;
use config::{Config, DockerConnectMode};

mod config;
mod container;
mod steam;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let docker_client = {
        #[cfg(unix)]
        match docker_connect_mode {
            DockerConnectMode::WindowsPipe => {
                panic!("Error: The docker daemon can't be connected to with a named pipe on unix")
            }
            DockerConnectMode::UnixSocket => Docker::connect_with_unix_defaults(),
            DockerConnectMode::Http => Docker::connect_with_http_defaults(),
            DockerConnectMode::SSL => Docker::connect_with_ssl_defaults(),
        }

        #[cfg(windows)]
        match docker_connect_mode {
            DockerConnectMode::UnixSocket => panic!(
                "Error: The docker daemon can't be connected to with a unix socket on windows"
            ),
            DockerConnectMode::WindowsPipe => Docker::connect_with_named_pipe_defaults(),
            DockerConnectMode::Http => Docker::connect_with_http_defaults(),
            DockerConnectMode::SSL => Docker::connect_with_ssl_defaults(),
        }
    }?;

    // Initialise all our containers
    for container in containers.iter_mut() {
        container.init(&key, &docker_client, &state_dir).await;
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
            container.update(&key, &docker_client).await;
            container.save_state(&state_dir);
        }
        info!("Sleeping for {} seconds", interval.as_secs());
        std::thread::sleep(interval);
    }
}
