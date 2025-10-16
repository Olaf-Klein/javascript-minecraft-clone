# Deployment Guide

## Local Development

### Prerequisites
- Node.js 18+ 
- npm 9+

### Setup

1. **Clone the repository**:
```bash
git clone https://github.com/Olaf-Klein/javascript-minecraft-clone.git
cd javascript-minecraft-clone
```

2. **Install dependencies**:
```bash
npm install
```

3. **Start the server**:
```bash
npm start
```

4. **Start the client (in another terminal)**:
```bash
npm run client:dev
```

5. **Access the game**:
Open your browser to `http://localhost:8080`

## Docker Deployment

### Build and Run

1. **Build the image**:
```bash
docker build -t minecraft-clone .
```

2. **Run the container**:
```bash
docker run -d \
  -p 3000:3000 \
  -v $(pwd)/worlds:/app/worlds \
  -v $(pwd)/server/plugins/plugins-data:/app/server/plugins/plugins-data \
  --name minecraft-server \
  minecraft-clone
```

3. **View logs**:
```bash
docker logs -f minecraft-server
```

### Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  server:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./worlds:/app/worlds
      - ./server/plugins/plugins-data:/app/server/plugins/plugins-data
      - ./server/config.json:/app/server/config.json
    environment:
      - NODE_ENV=production
    restart: unless-stopped
```

Run with:
```bash
docker-compose up -d
```

## Pterodactyl Deployment

### Prerequisites
- Pterodactyl panel installed
- Node.js egg or use the provided egg configuration

### Installation Steps

1. **Import the egg**:
   - Go to Admin Panel → Nests
   - Import `pterodactyl/egg.json`

2. **Create a server**:
   - Select the "JavaScript Minecraft Clone" egg
   - Configure server settings:
     - Memory: 2048 MB minimum
     - Disk: 5000 MB minimum
     - CPU: 100% minimum

3. **Configure variables**:
   - Server Port: 3000 (default)
   - Max Players: 20 (default)
   - World Name: world
   - World Seed: (optional)
   - Render Distance: 8

4. **Start the server**:
   - The server will automatically install dependencies
   - Wait for "Server is ready!" message

### Custom Configuration

Edit `server/config.json` in the file manager:

```json
{
  "port": 3000,
  "maxPlayers": 20,
  "renderDistance": 8,
  "worldName": "world",
  "seed": 12345,
  "gameMode": "creative",
  "difficulty": "peaceful",
  "pluginsEnabled": true,
  "serverName": "My Server",
  "motd": "Welcome!",
  "autoSave": true,
  "autoSaveInterval": 300000
}
```

## Production Deployment

### VPS/Dedicated Server

1. **Install Node.js**:
```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

2. **Clone and setup**:
```bash
git clone https://github.com/Olaf-Klein/javascript-minecraft-clone.git
cd javascript-minecraft-clone
npm install --production
```

3. **Configure the server**:
```bash
cp server/config.json server/config.production.json
nano server/config.production.json
```

4. **Use PM2 for process management**:
```bash
npm install -g pm2
pm2 start server/index.js --name minecraft-server
pm2 save
pm2 startup
```

5. **Setup reverse proxy (optional)**:

Nginx configuration:
```nginx
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    location /ws {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host $host;
    }
}
```

### Cloud Platforms

#### AWS EC2

1. Launch EC2 instance (t3.medium recommended)
2. Configure security groups:
   - Inbound: TCP 3000, 8080
3. Follow VPS setup instructions

#### Google Cloud

1. Create Compute Engine instance
2. Configure firewall rules for ports 3000, 8080
3. Follow VPS setup instructions

#### Heroku

Not recommended due to WebSocket requirements and ephemeral storage.

#### DigitalOcean

1. Create Droplet (2GB RAM minimum)
2. Follow VPS setup instructions

## Performance Tuning

### Server

1. **Adjust render distance**:
   - Lower values = better performance
   - Recommended: 6-10 chunks

2. **Enable auto-save interval**:
   - Balance between data safety and performance
   - Recommended: 300000ms (5 minutes)

3. **Limit max players**:
   - Based on available resources
   - 1GB RAM ≈ 5-10 players

### Client

1. **Build for production**:
```bash
npm run client:build
```

2. **Serve static files**:
Use nginx or another web server for the built client files.

## Monitoring

### Server Logs

```bash
# With PM2
pm2 logs minecraft-server

# With Docker
docker logs -f minecraft-server

# Direct
tail -f server.log
```

### Health Checks

The server exposes basic health information. You can monitor:
- Player count
- Memory usage
- Tick rate
- World save status

## Backup

### World Data

```bash
# Backup worlds directory
tar -czf backup-$(date +%Y%m%d).tar.gz worlds/

# Restore
tar -xzf backup-YYYYMMDD.tar.gz
```

### Automated Backups

Add to crontab:
```bash
0 2 * * * cd /path/to/minecraft-clone && tar -czf backups/backup-$(date +\%Y\%m\%d).tar.gz worlds/
```

## Troubleshooting

### Server won't start

1. Check port availability:
```bash
netstat -tulpn | grep 3000
```

2. Check Node.js version:
```bash
node --version  # Should be 18+
```

3. Check logs for errors

### High memory usage

1. Reduce render distance
2. Limit max players
3. Unload plugins
4. Check for memory leaks in plugins

### Connection issues

1. Verify firewall rules
2. Check WebSocket connection
3. Verify server address in client
4. Check for proxy/load balancer issues

## Scaling

### Horizontal Scaling (Future)

The architecture supports future horizontal scaling:
- Chunk servers for distributed world loading
- Redis for session management
- Load balancer for player distribution

### Vertical Scaling

Current recommendations:
- 2GB RAM: 10-20 players
- 4GB RAM: 30-50 players
- 8GB RAM: 50-100 players

## Security

1. **Enable firewall**:
```bash
sudo ufw allow 3000/tcp
sudo ufw enable
```

2. **Use HTTPS** (with reverse proxy)

3. **Regular updates**:
```bash
git pull
npm install
pm2 restart minecraft-server
```

4. **Monitor plugin sources**:
   - Only install trusted plugins
   - Review plugin code before installation

5. **Regular backups**:
   - Automated daily backups
   - Off-site backup storage
