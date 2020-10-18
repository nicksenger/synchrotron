FROM ekidd/rust-musl-builder AS build
WORKDIR /usr/src/
USER root

# install rustup/cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH

# Add compilation target for later scratch container
ENV RUST_TARGETS="x86_64-unknown-linux-musl"
RUN rustup target install x86_64-unknown-linux-musl

# Creating a placeholder project
RUN USER=root cargo new api-gateway
WORKDIR /usr/src/api-gateway

# moving deps info
COPY ./Cargo.lock ./Cargo.lock
COPY ./api-gateway/Cargo.toml ./Cargo.toml
COPY ./schema /usr/src/schema

# Caching deps
RUN cargo build --target x86_64-unknown-linux-musl --release

# Replacing with actual src
RUN rm /usr/src/api-gateway/src/*.rs
COPY ./api-gateway/src /usr/src/api-gateway/src

# Only code changes should need to compile
RUN cargo build --target x86_64-unknown-linux-musl --release

# This creates a tiny container with the executable
FROM scratch
COPY --from=build /usr/src/api-gateway/target/x86_64-unknown-linux-musl/release/api-gateway .
USER 1000
CMD ["./api-gateway"]
