# Node for Tanssi
#
# Requires to run from repository root and to copy the binary in the build folder (part of the release workflow)

FROM docker.io/library/ubuntu:22.04 AS builder

RUN apt-get update && apt-get install -y ca-certificates lsof && update-ca-certificates

FROM debian:bookworm-slim
LABEL maintainer "gorka@moondancelabs.com"
LABEL description="Binary for Dancelight"

RUN useradd -m -u 1000 -U -s /bin/sh -d /snowbridge-relay snowbridge-relay && \
	mkdir -p /snowbridge-relay/.local/share && \
	mkdir /data && \
	chown -R snowbridge-relay:snowbridge-relay /data && \
	ln -s /data /snowbridge-relay/.local/share/snowbridge-relay && \
	rm -rf /usr/sbin

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

USER snowbridge-relay

COPY --chown=snowbridge-relay build/snowbridge-relay /snowbridge-relay
RUN chmod uog+x /snowbridge-relay/snowbridge-relay

EXPOSE 30333 9933 9944 9615

VOLUME ["/data"]

ENTRYPOINT ["/snowbridge-relay/snowbridge-relay"]
