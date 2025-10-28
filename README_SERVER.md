# Dedicated server (Pterodactyl / Container)

This repository includes a small dedicated server binary (Rust) and containerization artifacts to run it inside Pterodactyl/Wings.

Files added:
- `Dockerfile.server` — multi-stage Dockerfile that builds the `dedicated-server` binary and produces a minimal runtime image.
- `start_server.sh` — tiny entrypoint wrapper that prints the bind/port and execs the binary.
- `pterodactyl-egg.json` — a simple Pterodactyl egg manifest that references a GHCR image and exposes a `PORT` variable.

Build & publish to GHCR (example)

1. Build locally:

```powershell
docker build -f Dockerfile.server -t ghcr.io/<OWNER>/minecraft-clone-server:latest .
```

2. Push to GHCR (example):

```powershell
docker login ghcr.io -u <USER> -p <TOKEN>
docker push ghcr.io/<OWNER>/minecraft-clone-server:latest
```

3. Import the `pterodactyl-egg.json` into your Pterodactyl panel (Admin → Nests → Import) and set the default image to the GHCR image above.

Runtime env
- `PORT` — port Pterodactyl will set (default 25565)
- `BIND` — bind address (default 0.0.0.0)

If you want me to create a GH Actions workflow to build & publish this image automatically, you said you already have one — I can still adapt it to use `Dockerfile.server` if you want.
