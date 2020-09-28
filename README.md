# Steam Docker Container Updater Service

<a href="https://travis-ci.com/github/Laura7089/gameserver-docker-updater">![Travis (.com)](https://img.shields.io/travis/com/laura7089/gameserver-docker-updater?style=flat-square)</a>
<a href="https://hub.docker.com/r/laura7089/steam-docker-updater">![Docker Pulls](https://img.shields.io/docker/pulls/laura7089/steam-docker-updater?style=flat-square)</a>
![GitHub](https://img.shields.io/github/license/laura7089/gameserver-docker-updater?style=flat-square)

A service to live-update dedicated servers for games running in docker containers.
It uses the steam API to check when version integers change, then initiates a user-defined action on the container.

## Usage

You will need a configuration file first.
It's recommended to use docker to launch the service:

```bash
docker run -d \
  -v "./updater-config.yml:/config.yml" \
  laura7089/steam-docker-updater
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
`action` | yes | The action to take when the container is out of date: currently only `restart` is supported, with `build` and `custom` planned
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
