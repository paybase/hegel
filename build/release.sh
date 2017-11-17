#!/bin/sh
set -e

sudo apt-get update
sudo apt-get install -y zip
filename=hegel-$CIRCLE_TAG-x86-linux.zip
zip $filename target/x86_64-unknown-linux-musl/release/hegel

curl -Lk https://github.com/tcnksm/ghr/releases/download/v0.5.4/ghr_v0.5.4_linux_amd64.zip -o ghr.zip
unzip ghr.zip

./ghr -u paybase -r hegel $CIRCLE_TAG $filename