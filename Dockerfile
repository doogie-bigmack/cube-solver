# Multi-stage Dockerfile for Rubik's Cube Solver
# Builds the WASM web application and serves it

# Stage 1: Build the Rust WASM application
FROM rust:1.92 as builder

# Install wasm-pack and other dependencies
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install dioxus-cli --version 0.6.3

WORKDIR /app

# Copy the project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

# Build the WASM application
RUN dx build --release --platform web

# Stage 2: Serve the application with nginx
FROM nginx:alpine

# Copy the built WASM files from builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration (if needed)
# COPY nginx.conf /etc/nginx/nginx.conf

# Expose port 8080
EXPOSE 8080

# Update nginx to listen on port 8080
RUN sed -i 's/listen       80;/listen       8080;/' /etc/nginx/conf.d/default.conf

CMD ["nginx", "-g", "daemon off;"]
