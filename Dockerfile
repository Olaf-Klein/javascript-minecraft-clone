# Dockerfile for JavaScript Minecraft Clone
FROM node:18-alpine

# Set working directory
WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm install --production

# Copy application files
COPY . .

# Create directories
RUN mkdir -p worlds server/plugins/plugins-data

# Expose default port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
  CMD node -e "require('http').get('http://localhost:3000', (r) => r.statusCode === 404 ? process.exit(0) : process.exit(1))"

# Start server
CMD ["npm", "start"]
