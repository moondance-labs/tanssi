# Node for Tanssi
#
# Requires to run from repository root and to copy the binary in the build folder (part of the release workflow)

FROM docker.io/library/ubuntu:20.04 AS builder

RUN apt-get update && apt-get install -y ca-certificates lsof && update-ca-certificates

FROM debian:bookworm-slim
LABEL maintainer "gorka@moondancelabs.com"
LABEL description="Binary for Dancelight"

RUN useradd -m -u 1000 -U -s /bin/sh -d /tanssi-relay tanssi-relay && \
	mkdir -p /tanssi-relay/.local/share && \
	mkdir /data && \
	chown -R tanssi-relay:tanssi-relay /data && \
	ln -s /data /tanssi-relay/.local/share/tanssi-relay && \
	rm -rf /usr/sbin

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

USER tanssi-relay

COPY --chown=tanssi-relay build/tanssi-relay* /tanssi-relay
RUN chmod uog+x /tanssi-relay/tanssi-relay*

# 30333 for parachain p2p
# 30334 for relaychain p2p
# 30335 for container p2p
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
# 9935 for RPC call container (if we want to expose this)
# 9946 for Websocket container (if we want to expose this)
# 9617 for Prometheus container (metrics)
EXPOSE 30333 30334 30335 9933 9944 9615 9935 9946 9617

VOLUME ["/data"]

ENTRYPOINT ["/tanssi-relay/tanssi-relay"]
