# Node for Container-chain-simple-template
#
# Requires to run from repository root and to copy the binary in the build folder (part of the release workflow)

FROM docker.io/library/ubuntu:20.04 AS builder

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

FROM debian:bookworm-slim
LABEL maintainer "gorka@moondancelabs.com"
LABEL description="Binary for simple container chain template node"

RUN useradd -m -u 1000 -U -s /bin/sh -d /container-chain-template-simple container-chain-template-simple && \
	mkdir -p /container-chain-template-simple/.local/share && \
	mkdir /data && \
	chown -R container-chain-template-simple:container-chain-template-simple /data && \
	ln -s /data /container-chain-template-simple/.local/share/container-chain-template-simple && \
	rm -rf /usr/sbin

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

USER container-chain-template-simple

COPY --chown=container-chain-template-simple build/container-chain-simple-node* /container-chain-template-simple
RUN chmod uog+x /container-chain-template-simple/container-chain-simple*

# 30333 for parachain p2p
# 30334 for relaychain p2p
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 30334 9933 9944 9615

VOLUME ["/data"]

ENTRYPOINT ["/container-chain-template-simple/container-chain-simple-node"]