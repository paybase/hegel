#!/bin/sh
cargo run -- \
  -p "sh ./test/test.sh 5" \
  -p "sh ./test/test.sh 10"

if [ $? = 1 ]; then
  echo "test successful"
else 
  echo "test unsucessful"
  exit 1
fi
