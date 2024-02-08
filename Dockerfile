FROM rust:1.76 as build

# from https://github.com/clux/muslrust/blob/06dc8cc98164208a495fedbcff2e93ff498ac3ce/Dockerfile#L15-L36
RUN apt-get update && apt-get install -y \
    musl-dev \
    musl-tools \
    file \
    git \
    openssh-client \
    make \
    cmake \
    g++ \
    curl \
    pkgconf \
    ca-certificates \
    xutils-dev \
    libssl-dev \
    libpq-dev \
    automake \
    autoconf \
    libtool \
    protobuf-compiler \
    libprotobuf-dev \
    --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

# create a new empty shell project
RUN USER=root cargo new --bin webhook-listener
WORKDIR /webhook-listener

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

## Install target platform (Cross-Compilation)
RUN rustup target add x86_64-unknown-linux-musl

# this build step will cache your dependencies
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/webhook_listener*
RUN cargo build --target x86_64-unknown-linux-musl --release


# our final base
FROM alpine:3.17 as runner

# copy the build artifact from the build stage
COPY --from=build /webhook-listener/target/x86_64-unknown-linux-musl/release/webhook-listener /

# set the startup command to run your binary
CMD ["/webhook-listener"]
