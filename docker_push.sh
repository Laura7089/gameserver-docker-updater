#!/bin/sh

echo "$DOCKER_PASSWORD" | docker login -u laura7089 --password-stdin
docker push laura7089/heat-exchanger
