# Node for Tanssi
FROM docker.io/library/ubuntu:20.04 AS builder

RUN apt-get update && apt-get install -y ca-certificates lsof && update-ca-certificates

# Final stage with Ubuntu 24.04 for newer GLIBC
FROM ubuntu:24.04
LABEL maintainer "gorka@moondancelabs.com"
LABEL description="Binary for Tanssi Collator"

RUN useradd -m -u 2000 -U -s /bin/sh -d /tanssi tanssi && \
    mkdir -p /tanssi/.local/share && \
    mkdir /data && \
    chown -R tanssi:tanssi /data && \
    ln -s /data /tanssi/.local/share/tanssi && \
    rm -rf /usr/sbin

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

USER tanssi

COPY --chown=tanssi build/tanssi-node* /tanssi
RUN chmod uog+x /tanssi/tanssi*

EXPOSE 30333 30334 30335 9933 9944 9615 9935 9946 9617
VOLUME ["/data"]
ENTRYPOINT ["/tanssi/tanssi-node"]