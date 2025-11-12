# Node for Tanssi
#
# Requires to run from repository root and to copy the binary in the build folder (part of the release workflow)

FROM docker.io/library/ubuntu:20.04 AS builder

RUN apt-get update && apt-get install -y ca-certificates lsof && update-ca-certificates

FROM debian:bookworm-slim
LABEL maintainer="gorka@moondancelabs.com"
LABEL description="Binary for Dancelight"

# Create runtime user and data dirs
RUN useradd -m -u 1000 -U -s /bin/sh -d /tanssi-relay tanssi-relay && \
	mkdir -p /tanssi-relay/.local/share && \
	mkdir /data && \
	chown -R tanssi-relay:tanssi-relay /data && \
	ln -s /data /tanssi-relay/.local/share/tanssi-relay && \
	rm -rf /usr/sbin

# CA bundle from builder stage
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# Install binaries to /usr/local/bin
COPY build/tanssi-relay* /usr/local/bin/
RUN chmod uog+x /usr/local/bin/tanssi-relay* && \
    # For backwards compatibility: symlink all binaries into the old location
    for f in /usr/local/bin/tanssi-relay*; do \
        ln -sf "$f" "/tanssi-relay/$(basename "$f")"; \
    done \

# Drop privileges for runtime
USER tanssi-relay

EXPOSE 30333 9933 9944 9615

VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/tanssi-relay"]
