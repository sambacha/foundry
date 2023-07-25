# syntax=docker/dockerfile:1.4

FROM alpine:3.17@sha256:124c7d2707904eea7431fffe91522a01e5a861a624ee31d03372cc1d138a3126 AS build-environment

ARG TARGETARCH
WORKDIR /opt

RUN apk add --no-cache -t .build-deps clang lld curl build-base linux-headers git \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh \
    && chmod +x ./rustup.sh \
    && ./rustup.sh -y

RUN [[ "$TARGETARCH" = "arm64" ]] && echo "export CFLAGS=-mno-outline-atomics" >> $HOME/.profile || true

WORKDIR /opt/foundry
COPY . .

RUN --mount=type=cache,target=/root/.cargo/git/db \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/registry/index \
    cargo fetch
ARG TARGETPLATFORM
RUN --mount=type=cache,target=/root/.cargo/git/db \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/registry/index \
    --mount=type=cache,target=/opt/foundry/target \
    source $HOME/.profile && cargo build --release \
    
RUN set -eux; \
	\
    mkdir out \
    && mv target/release/forge out/forge \
    && mv target/release/cast out/cast \
    && mv target/release/anvil out/anvil \
    && mv target/release/chisel out/chisel \
    && strip out/forge \
    && strip out/cast \
    && strip out/chisel \
    && strip out/anvil; \
    rm -rf target/*; \
    apk del --purge .build-deps;

FROM docker.io/frolvlad/alpine-glibc:alpine-3.16_glibc-2.34 as foundry-client
RUN addgroup -g 10001 -S foundry && adduser -u 10000 -S -G foundry -h /opt/foundry foundry

RUN apk upgrade
RUN apk add --no-cache linux-headers git

COPY --from=build-environment /opt/foundry/out/forge /usr/local/bin/forge
COPY --from=build-environment /opt/foundry/out/cast /usr/local/bin/cast
COPY --from=build-environment /opt/foundry/out/anvil /usr/local/bin/anvil
COPY --from=build-environment /opt/foundry/out/chisel /usr/local/bin/chisel
ARG WORKSPACE_DIR=/workspace



# see https://github.blog/2022-04-12-git-security-vulnerability-announced/
RUN git config --global --add safe.directory ${WORKSPACE_DIR}

# Build doc by default
WORKDIR ${WORKSPACE_DIR}

ENTRYPOINT ["/bin/sh", "-c"]

LABEL github.workflow=${GITHUB_WORKFLOW}
LABEL github.runid=${GITHUB_RUN_ID}
LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.name="Foundry" \
      org.label-schema.description="Foundry" \
      org.label-schema.url="https://getfoundry.sh" \
      org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://github.com/foundry-rs/foundry.git" \
      org.label-schema.vendor="Foundry-rs" \
      org.label-schema.version=$VERSION \
      org.label-schema.schema-version="1.0"
