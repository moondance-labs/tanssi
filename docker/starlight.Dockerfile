# Node for Tanssi
#
# Requires to run from repository root and to copy the binary in the build folder (part of the release workflow)

FROM docker.io/library/ubuntu:20.04 AS builder

RUN apt-get update && apt-get install -y ca-certificates lsof && update-ca-certificates

FROM ubuntu:24.04
LABEL maintainer "gorka@moondancelabs.com"
LABEL description="Binary for Dancelight"

RUN useradd -m -u 2000 -U -s /bin/sh -d /tanssi-relay tanssi-relay && \
	mkdir -p /tanssi-relay/.local/share && \
	mkdir /data && \
	chown -R tanssi-relay:tanssi-relay /data && \
	ln -s /data /tanssi-relay/.local/share/tanssi-relay && \
	rm -rf /usr/sbin

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

USER tanssi-relay

COPY --chown=tanssi-relay build/tanssi-relay* /tanssi-relay
RUN chmod uog+x /tanssi-relay/tanssi-relay*

EXPOSE 30333 9933 9944 9615

VOLUME ["/data"]

ENTRYPOINT ["/tanssi-relay/tanssi-relay"]
