FROM scratch
COPY --from=synchrotron_base:latest /usr/src/microbiome/target/x86_64-unknown-linux-musl/release/users-service .
USER 1000
CMD ["./users-service"]
