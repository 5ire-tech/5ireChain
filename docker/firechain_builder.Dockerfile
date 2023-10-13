FROM rust:nightly AS builder


WORKDIR /firechain


COPY . .


RUN cargo build --release


FROM debian:bullseye-slim


RUN apt-get update && apt-get install -y --no-install-recommends curl git openssh-client && \
    apt-get clean && rm -rf /var/lib/apt/lists/*


COPY --from=builder /firechain/target/release/firechain-node /usr/local/bin/firechain-node

# Create a non-root user to run the application
RUN useradd -m -u 1000 -U -s /bin/sh -d /firechain firechain

# Create necessary directories and set permissions
RUN mkdir -p /data /firechain/.local/share/firechain && \
    chown -R firechain:firechain /data && \
    ln -s /data /firechain/.local/share/firechain

# Expose necessary ports
EXPOSE 30333 9933 9944 9946

# Switch to the non-root user
USER firechain

# Set the entry point
CMD ["firechain-node"]
