# Heat Exchanger

<a href="https://travis-ci.com/github/Laura7089/gameserver-docker-updater">![Travis (.com)](https://img.shields.io/travis/com/laura7089/gameserver-docker-updater?style=flat-square)</a>
<a href="https://hub.docker.com/r/laura7089/heat-exchanger">![Docker Pulls](https://img.shields.io/docker/pulls/laura7089/heat-exchanger?style=flat-square)</a>
<a href="https://github.com/Laura7089/heat-exchanger">![GitHub last commit (branch)](https://img.shields.io/github/last-commit/laura7089/heat-exchanger/master?style=flat-square)</a>
![GitHub](https://img.shields.io/github/license/laura7089/heat-exchanger?style=flat-square)

Keep your steam games' dedicated servers hot!

A service to live-update dedicated servers for steam games running in docker containers.
It uses the steam API to check when version integers change, then initiates a user-defined action on the container.

## Usage

You will need a configuration file first.
It's recommended to use docker to launch the service:

```bash
docker run -d \
  -v "./updater-config.yml:/config.yml" \
  -v "/var/run/docker.sock:/var/run/docker.sock" \
  laura7089/heat-exchanger
```

## Configuration

The service is configured with a `.yml` config file, specified with any of the following:

1. Passed as the first argument to the service
2. Passed with the `UPDATER_CONFIG_PATH` env var
3. Allowed to default to `./config.yml` when running native or `/config.yml` in docker

Option | Required | Description
---|---|---
`steam_api_key` | no | Your steam API key, found at https://steamcommunity.com/dev/apikey, can also be set with the `UPDATER_STEAM_API_KEY` environment variable
`check_interval` | yes | The time to wait between checking for updates, in [humantime format](https://docs.rs/humantime/2.0.1/humantime/index.html)
`containers` | yes | The list of containers to monitor + other options
`state_directory` | no | The directory in which to store the state of containers on disk, defaults to `./state` running native or `/updater_state` in docker

The containers are configured as follows:

Option | Required | Description
---|---|---
`name` | yes | The name or identifier of the container in docker
`appid` | yes | The steam appid of the game the service is running, this is unfortunately limited to the appid of the client since steam don't appear to publish versions for products that don't have a steam store page
`action` | yes | The action to take when the container is out of date: currently only `restart` is supported, with `build`, `pull` and `custom` planned
`options` | no | Key-value map of extra information required by the `action` you have chosen, currently not in use

An example file could be:

```yaml
---

steam_api_key: "AW3RAWL4HBFWL43HBAWL3HB-your-api-key-here"
check_interval: "2h"

containers:
  - name: "scpsl"
    appid: 700330
    action: build
    options:
      context: /builds/scpsl
  - name: "tf2"
    appid: 440
    action: restart
```

## Logging

Set the environment variable `RUST_LOG` to change the logging level:

- `debug` will print as much information as possible - this will print your API key to the console!
- `info` will print enough information to get a good idea of what the program is doing - this is the default when running in docker
- `warn` will print only warnings and errors
- `error` will print only errors
