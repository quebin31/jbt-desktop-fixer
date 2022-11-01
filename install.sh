#!/bin/bash

# Install script version: 1.0.0

readonly VERSION="1.0.0"
readonly DEST=${1:-/usr/local/bin}
readonly BIN="jbt-desktop-fixer"
readonly URL="https://github.com/quebin31/jbt-desktop-fixer/releases/download/${VERSION}/${BIN}"

curl -sL "${URL}" -o "${BIN}" 
install -Dm755 "${BIN}" "${DEST}"