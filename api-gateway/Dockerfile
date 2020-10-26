FROM scratch
COPY --from=microbiome_base:latest /usr/src/microbiome/target/x86_64-unknown-linux-musl/release/gateway .
WORKDIR /static
COPY --from=microbiome_base:latest /usr/src/microbiome/frontend/index.html .
WORKDIR /static/pkg
COPY --from=microbiome_base:latest /usr/src/microbiome/frontend/pkg .
WORKDIR /
USER 1000
CMD ["./gateway"]
