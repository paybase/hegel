#!/bin/sh

set -e

version=`cat Cargo.toml | sed -n -e 's/.*version = \"\(.*\)\".*/\1/p'`

echo "creating tag: $version"
git tag $version
git push origin $version
