#!/bin/sh

err() {
  echo "$@" 1>&2
}

log() {
  echo "$@"
}

if [ -z "$1" ]; then
  iterations=5
else
  iterations=$1
fi

err "check stderr"

i=1
while [ "$i" -le "$iterations" ]; do
  log "iteration: $i"
  sleep 1
  i=$(($i + 1))
done
