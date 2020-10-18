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
RUN USER=root cargo new users-service
WORKDIR /usr/src/users-service

# moving deps info
COPY ./Cargo.lock ./Cargo.lock
COPY ./users-service/Cargo.toml ./Cargo.toml
COPY ./schema /usr/src/schema

# Caching deps
RUN cargo build --target x86_64-unknown-linux-musl --release

# Replacing with actual src
RUN rm /usr/src/users-service/src/*.rs
COPY ./users-service/src /usr/src/users-service/src

# Only code changes should need to compile
RUN cargo build --target x86_64-unknown-linux-musl --release

# This creates a tiny container with the executable
FROM scratch
COPY --from=build /usr/src/users-service/target/x86_64-unknown-linux-musl/release/users-service .
USER 1000
CMD ["./users-service"]
