FROM scratch
COPY --from=microbiome_base:latest /usr/src/microbiome/target/x86_64-unknown-linux-musl/release/api-gateway .
RUN mkdir static
COPY --from=microbiome_base:latest /usr/src/microbiome/frontend/index.html ./static
COPY --from=microbiome_base:latest /usr/src/microbiome/frontend/pkg ./static
USER 1000
CMD ["./api-gateway"]
