#!/bin/bash
cargo build -r
mkdir output
mkdir output/App
mkdir output/App/bin
cp target/release/discord_presence output/App/bin/discord_presence
cp -r assets output/App/
