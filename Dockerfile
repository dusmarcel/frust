# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.80.1
ARG APP_NAME=frust

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
RUN cargo install trunk
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=index.html,target=index.html \
    --mount=type=bind,source=tailwind.css,target=tailwind.css \
    trunk build --release

FROM nginx:latest AS final
COPY --from=build /app/dist /usr/share/nginx/html