# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.74.0
ARG APP_NAME=rs-search-engine

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git
COPY ./cache/ /app/cache/

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

# Extract the pages archive and index them
RUN tar -xvf /app/cache/pages.tar.gz -C /app
RUN /bin/server index file

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.

FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Copy the cache folder and frontend folder from the host to the container
COPY cache /bin/cache
COPY frontend /bin/frontend

# Setting the working directory to /bin
WORKDIR /bin/

# Remove pages folder after indexed
RUN rm -rf ./pages/

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["/bin/server", "serve"]