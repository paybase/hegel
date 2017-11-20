#!/bin/sh
set -e

if [ -z "$CIRCLE_TAG" ] && [ "$CIRCLE_BRANCH" = master ]; then
  sh -c "build/tag.sh"
elif [ -n "$CIRCLE_TAG" ]; then
  cargo build --release
  sh -c "build/release.sh"
else
  echo "not doing anything here!"
fi
