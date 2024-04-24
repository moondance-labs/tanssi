# Node for Container-chain-evm-template
#
# Requires to run from repository root and to copy the binary in the build folder (part of the release workflow)

FROM docker.io/library/ubuntu:20.04 AS builder

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

FROM debian:bookworm-slim
LABEL maintainer "gorka@moondancelabs.com"
LABEL description="Binary for container-chain-template-evm Collator"

RUN useradd -m -u 1000 -U -s /bin/sh -d /container-chain-template-evm container-chain-template-evm && \
	mkdir -p /container-chain-template-evm/.local/share && \
	mkdir /data && \
	chown -R container-chain-template-evm:container-chain-template-evm /data && \
	ln -s /data /container-chain-template-evm/.local/share/container-chain-template-evm && \
	rm -rf /usr/sbin

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

USER container-chain-template-evm

COPY --chown=container-chain-template-evm build/container-chain-frontier-node* /container-chain-template-evm
RUN chmod uog+x /container-chain-template-evm/container-chain-frontier*

# 30333 for parachain p2p
# 30334 for relaychain p2p
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 30334 9933 9944 9615

VOLUME ["/data"]

ENTRYPOINT ["/container-chain-template-evm/container-chain-frontier-node"]