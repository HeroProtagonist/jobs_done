#!/bin/sh

echo Installing binary...

machine=$(uname -sm)

case $machine in
  "Darwin x86_64")
    asset=macos-64-bit
    ;;
  "Darwin arm64")
    asset=macos-arm
    ;;
  "Linux x86_64")
    asset=linux-64-bit
    ;;
  *)
    echo "unknown machine - PRs welcome"
    exit 1
    ;;
esac

tmp=$(mktemp -d)

url="https://github.com/HeroProtagonist/jobs_done/releases/latest/download/$asset.zip"

curl -sL $url | tar -C $tmp -xz

dest=/usr/local/bin

install -m 755 $tmp/jobs_done $dest

rm -rf $tmp
