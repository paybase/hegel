#!/bin/sh
set -e

if [ -z "$CIRCLE_TAG" ] && [ "$CIRCLE_BRANCH" = master ]; then
  . ./tag.sh
elif [ -n "$CIRCLE_TAG" ]; then
  . ./release.sh
fi