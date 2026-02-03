FROM rust:1 AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY packages ./packages
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Install build deps (GTK for Dioxus Desktop, Node prep, curl for scripts)
RUN apt update && apt install -y \
    libgtk-3-dev \
    libatk1.0-dev \
    libgdk-pixbuf-xlib-2.0-dev \
    libpango-1.0-0 \
    libpango1.0-dev \
    libglib2.0-dev \
    pkg-config \
    libwebkit2gtk-4.1-dev \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    binaryen \
    curl \
    ca-certificates \
    gnupg \
    && rm -rf /var/lib/apt/lists/*

RUN cargo chef cook --recipe-path recipe.json

# Node + Tailwind
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
   && apt update \
   && apt install -y nodejs \
   && rm -rf /var/lib/apt/lists/*
RUN npm init -y
RUN npm install -D tailwindcss @tailwindcss/cli

# Dioxus CLI
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    cargo binstall dioxus-cli -y --force

ENV PATH="/root/.cargo/bin:$PATH"
ENV RUSTFLAGS="-C debuginfo=0"

RUN rustup target add wasm32-unknown-unknown

COPY . .

# Tailwind build
# RUN npx tailwindcss \
#   -i ./packages/ui/input.css \
#   -o ./packages/ui/assets/output.css \
#   --minify

# RUN npx tailwindcss \
#   -i ./packages/web/input.css \
#   -o ./packages/web/assets/output.css \
#   --minify

# Dioxus bundle
RUN dx bundle --package web --release

FROM nginx:alpine AS runtime
COPY --from=builder /app/target/dx/web/release/web/public/ /usr/share/nginx/html/

# Configure nginx for SPA routing
RUN echo 'server { \
    listen 8080; \
    root /usr/share/nginx/html; \
    index index.html; \
    location / { \
        try_files $uri $uri/ /index.html; \
    } \
    location ~* \.(wasm|js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ { \
        expires 1y; \
        add_header Cache-Control "public, immutable"; \
    } \
    gzip on; \
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript application/wasm; \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
