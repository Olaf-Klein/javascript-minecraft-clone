#!/usr/bin/env sh
# Simple entrypoint wrapper that honors PORT and BIND environment variables
set -euo pipefail

: "${PORT:=25565}"
: "${BIND:=0.0.0.0}"

echo "Starting dedicated server on ${BIND}:${PORT}"
exec /usr/local/bin/dedicated-server
