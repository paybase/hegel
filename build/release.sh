#!/bin/sh
set -e

ensure_sha256() {
    if ! openssl dgst -sha256 "$1" | grep "$2"; then
        log "SHA256 for $1 did not match expected $2"
        exit 1
    fi
}

sudo apt-get update
sudo apt-get install -y zip

GHR_VERSION=0.5.4
GHR_SHA256=6f3cff97000fc643019e66f13252300afd531fea347e61949fedfba7e20405a1
GHR_URL=https://github.com/tcnksm/ghr/releases/download/v${GHR_VERSION}/ghr_v${GHR_VERSION}_linux_amd64.zip

filename=hegel-$CIRCLE_TAG-x86-linux.tar.gz
mkdir -p release
tar -zcf release/$filename -C target/x86_64-unknown-linux-musl/release hegel
echo "$(openssl dgst -sha256 "release/$filename" | cut -d' ' -f2) - $filename" > release/SHASUM

curl -Lk $GHR_URL -o ghr.zip
ensure_sha256 ghr.zip $GHR_SHA256
unzip ghr.zip

./ghr -u paybase -r hegel $CIRCLE_TAG release/
