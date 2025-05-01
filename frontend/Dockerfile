# Dockerfile for Frontend (Svelte)
FROM node:18 as builder

WORKDIR /app
COPY . .

# Install dependencies and build the frontend
RUN npm install && npm run build

# Create a minimal image for serving the frontend
FROM nginx:alpine
WORKDIR /usr/share/nginx/html

# Copy the build output to the nginx html directory
COPY --from=builder /app/public/build .

# Expose the port nginx runs on
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]