#!/bin/zsh
docker run -w "/app" -v "$PWD:/app" --rm -it rust:latest bash -c "cargo build --release"