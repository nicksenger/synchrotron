FROM ekidd/rust-musl-builder
WORKDIR /usr/src/
USER root

# install rustup/cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2020-10-25
ENV PATH /root/.cargo/bin:$PATH

# Add compilation target for later scratch container
ENV RUST_TARGETS="x86_64-unknown-linux-musl"
RUN rustup target install x86_64-unknown-linux-musl

# Creating a placeholder workspace
RUN USER=root mkdir microbiome
WORKDIR /usr/src/microbiome
RUN USER=root cargo install cargo-make
RUN USER=root cargo new api-gateway
WORKDIR /usr/src/microbiome/api-gateway/src
RUN touch lib.rs
WORKDIR /usr/src/microbiome
RUN USER=root cargo new schema --lib
RUN USER=root cargo new users-service
RUN USER=root cargo new courses-service
COPY ./api-gateway/Cargo.toml /usr/src/microbiome/api-gateway/Cargo.toml
COPY ./schema/Cargo.toml /usr/src/microbiome/schema/Cargo.toml
COPY ./users-service/Cargo.toml /usr/src/microbiome/users-service/Cargo.toml
COPY ./courses-service/Cargo.toml /usr/src/microbiome/courses-service/Cargo.toml
COPY ./Cargo.toml /usr/src/microbiome/Cargo.toml
COPY ./Cargo.lock /usr/src/microbiome/Cargo.lock

# Caching deps
RUN cargo build --target x86_64-unknown-linux-musl --release

# Replacing with actual src
COPY ./api-gateway /usr/src/microbiome/api-gateway
COPY ./schema /usr/src/microbiome/schema
COPY ./users-service /usr/src/microbiome/users-service
COPY ./courses-service /usr/src/microbiome/courses-service
RUN sed -i 's/localhost/host.docker.internal/g' /usr/src/microbiome/users-service/.env
RUN sed -i 's/localhost/host.docker.internal/g' /usr/src/microbiome/courses-service/.env

# Only code changes should need to compile
RUN cargo build --target x86_64-unknown-linux-musl --release -p users-service
RUN cargo build --target x86_64-unknown-linux-musl --release -p courses-service
RUN cargo build --target x86_64-unknown-linux-musl --release -p api-gateway

CMD echo ""
