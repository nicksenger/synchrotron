FROM scratch
COPY --from=synchrotron_base:latest /usr/src/microbiome/target/x86_64-unknown-linux-musl/release/courses-service .
USER 1000
CMD ["./courses-service"]
