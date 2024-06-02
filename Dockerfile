# syntax=docker/dockerfile:1.4
FROM alpine:3.19 as build-environment

ARG TARGETARCH
WORKDIR /opt

RUN apk add --no-cache clang lld curl build-base linux-headers git \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh \
    && chmod +x ./rustup.sh \
    && ./rustup.sh -y

RUN [[ "$TARGETARCH" = "arm64" ]] && echo "export CFLAGS=-mno-outline-atomics" >> $HOME/.profile || true

WORKDIR /opt/foundry
COPY . .

# see <https://github.com/foundry-rs/foundry/issues/7925> git config --local index.skipHash false; git reset --mixed
RUN git update-index --force-write-index

RUN --mount=type=bind,source=src,target=/opt/foundry \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/opt/foundry/target \
    source $HOME/.profile && cargo build --release \
    && mkdir out \
    && mv target/release/forge out/forge \
    && mv target/release/cast out/cast \
    && mv target/release/anvil out/anvil \
    && mv target/release/chisel out/chisel \
    && strip out/forge \
    && strip out/cast \
    && strip out/chisel \
    && strip out/anvil;

FROM docker.io/frolvlad/alpine-glibc:alpine-3.19_glibc-2.34 as foundry-client

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    foundry

USER foundry

RUN apk add --no-cache linux-headers git

COPY --from=build-environment /opt/foundry/out/forge /usr/local/bin/forge
COPY --from=build-environment /opt/foundry/out/cast /usr/local/bin/cast
COPY --from=build-environment /opt/foundry/out/anvil /usr/local/bin/anvil
COPY --from=build-environment /opt/foundry/out/chisel /usr/local/bin/chisel


ENTRYPOINT ["/bin/sh", "-c"]

LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.name="Foundry" \
      org.label-schema.description="Foundry" \
      org.label-schema.url="https://getfoundry.sh" \
      org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://github.com/foundry-rs/foundry.git" \
      org.label-schema.vendor="Foundry-rs" \
      org.label-schema.version=$VERSION \
      org.label-schema.schema-version="1.0"
