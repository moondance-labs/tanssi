ARG SRTOOL_IMAGE_TAG
ARG SRTOOL_IMAGE_REPO

FROM ${SRTOOL_IMAGE_REPO}:${SRTOOL_IMAGE_TAG}

USER root

RUN apt-get update && \
    apt-get install openssh-server -y

USER 1001