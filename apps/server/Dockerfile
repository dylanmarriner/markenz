FROM node:18-alpine AS builder

WORKDIR /app

# Copy package files
COPY apps/server/package*.json ./
COPY apps/server/tsconfig.json ./

# Install all dependencies (including dev dependencies for build)
RUN npm ci

# Copy source code
COPY apps/server/src ./src

# Build the application
RUN npm run build

# Production stage
FROM node:18-alpine AS production

WORKDIR /app

# Copy package files and install production dependencies only
COPY apps/server/package*.json ./
RUN npm ci --only=production && npm cache clean --force

# Copy built application
COPY --from=builder /app/dist ./dist

# Create non-root user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S markenz -u 1001

# Change ownership of the app directory
RUN chown -R markenz:nodejs /app
USER markenz

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD node -e "require('http').get('http://localhost:3000/health', (res) => { process.exit(res.statusCode === 200 ? 0 : 1) })"

EXPOSE 3000 3001

CMD ["npm", "start"]
