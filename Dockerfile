FROM rust:1.69 as build

# create a new empty shell project
RUN USER=root cargo new --bin webhook-listener
WORKDIR /webhook-listener

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/webhook_listener*
RUN cargo build --release


# our final base
FROM gcr.io/distroless/static-debian11 as runner

# copy the build artifact from the build stage
COPY --from=build /webhook-listener/target/release/webhook-listener .

# set the startup command to run your binary
CMD ["./webhook-listener"]
