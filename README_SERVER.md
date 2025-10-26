Multiplayer server
------------------

![Build status](https://github.com/Olaf-Klein/javascript-minecraft-clone/actions/workflows/docker-publish.yml/badge.svg?branch=alternate-version)

Multiplayer server
------------------

This repository contains a minimal TCP-based multiplayer server binary (`minecraft-clone-server`) intended to be run separately from the game client. It's designed so you can containerize it and run it on Pterodactyl or any Docker host.

Quick start (local build):

```bash
cargo build --release --bin minecraft-clone-server
./target/release/minecraft-clone-server
```

Default listen address: `0.0.0.0:25565` (change with `SERVER_ADDR` env var)

Docker (build & run):

```bash
docker build -f Dockerfile.server -t mc-clone-server:latest .
docker run -p 25565:25565 -e SERVER_ADDR=0.0.0.0:25565 mc-clone-server:latest
```

Pterodactyl notes
- Image: use the Dockerfile.server to produce an image and create an Egg that runs `./minecraft-clone-server`.
- Expose TCP port 25565 in the egg allocation.

Continuous builds (GitHub Container Registry)
------------------------------------------

This repository includes a GitHub Actions workflow that can build `Dockerfile.server` and publish it to GitHub Container Registry (GHCR) on pushes to `main`.

How it works:

- The workflow is defined in `.github/workflows/docker-publish.yml`.
- By default it uses the repository's `GITHUB_TOKEN` to authenticate to GHCR. To allow the workflow to publish packages using `GITHUB_TOKEN`, ensure the repository settings allow `packages: write` for workflow permissions (this is enabled in the workflow via `permissions: packages: write`).

If you prefer to use a Personal Access Token (PAT), create a token with `write:packages` and `read:packages` and add it to the repository secrets as `GHCR_PAT`. The repository already includes a workflow configured to use this secret. Steps:

1. Create a PAT on GitHub:
	- Settings → Developer settings → Personal access tokens → Tokens (classic) → Generate new token.
	- Give it the `write:packages` and `read:packages` scopes (and `repo` if the repository is private).
	- Copy the token value (you won't see it again).

2. Add the token to your repository secrets:
	- Repo → Settings → Secrets and variables → Actions → New repository secret.
	- Name: `GHCR_PAT`
	- Value: paste the PAT from step 1.

3. Push or merge to the `alternate-version` branch — the workflow will use `GHCR_PAT` to authenticate and publish the image.

Note: the workflow will fail if `GHCR_PAT` is not present; this is intentional to avoid permission issues when pushing to GHCR.

After the workflow runs you will have images at:

- `ghcr.io/<your-org-or-username>/mc-clone-server:latest`
- `ghcr.io/<your-org-or-username>/mc-clone-server:<sha>`

You can then update the egg `container.image` to point to `ghcr.io/<your-org-or-username>/mc-clone-server:latest` so Pterodactyl/Wings will pull the image from GHCR when launching the server.

