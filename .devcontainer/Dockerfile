FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

ARG PROTOC_VERSION=28.2
ARG GRPCURL_VERSION=1.9.1

RUN set -eux; \
    \
    # Install protoc
    curl -sSL -o protoc.zip \
        https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip; \
    unzip -o protoc.zip -x readme.txt -d /usr/local; \
    rm -f protoc.zip; \
    \
    # Install grpcurl
    curl -sSL -o grpcurl.tar.gz \
        https://github.com/fullstorydev/grpcurl/releases/download/v${GRPCURL_VERSION}/grpcurl_${GRPCURL_VERSION}_linux_x86_64.tar.gz; \
    tar -C /usr/local/bin -zxf grpcurl.tar.gz grpcurl; \
    rm -f grpcurl.tar.gz; \
    \
    # List binary
    ls -l /usr/local/bin
