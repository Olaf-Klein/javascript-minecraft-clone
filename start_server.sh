#!/usr/bin/env bash
# Simple startup script used by Pterodactyl (or local testing)
export SERVER_ADDR=${SERVER_ADDR:-0.0.0.0:25565}
exec ./minecraft-clone-server
