# syntax=docker/dockerfile-upstream:master-experimental
FROM rust:1.61.0-slim-bullseye as build

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -qqy --no-install-recommends \
    gcc \
    libssl-dev \
    build-essential \
    dpkg-sig \
    libcap-dev \
    libc6-dev \
    librust-pkg-config-dev \
    openssl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false

WORKDIR /opt/foundry

ENV CARGO_HOME=/opt/foundry/.cargo

COPY . /opt/foundry

# copy over your manifests
COPY ./Cargo.toml ./opt/foundry/Cargo.toml

COPY ./Cargo.lock ./opt/foundry/Cargo.lock
# cargo install --path ./cli --bins --locked --force --root $FOUNDRY_DIR
# -C target-cpu=native
# RUSTFLAGS="-Clink-arg=-fuse-ld=lld" cargo build --release
# RUSTFLAGS="-C target cpu=native"
RUN cargo build --bins --release --target x86_64-unknown-linux-gnu

# build for release
RUN rm -rf ./target/release/x86_64-unknown-linux-gnu/deps/
RUN rm -rf ./target/release/x86_64-unknown-linux-gnu/build/
RUN strip target/x86_64-unknown-linux-gnu/release/anvil

FROM gcr.io/distroless/cc:debug as build-release

EXPOSE 8545/tcp
EXPOSE 8545/udp
EXPOSE 8180
EXPOSE 3001/tcp

# copy the build artifact from the build stage
COPY --chmod=0744 --from=build /opt/foundry/target/x86_64-unknown-linux-gnu/release/forge /usr/local/bin/forge
COPY --chmod=0744 --from=build /opt/foundry/target/x86_64-unknown-linux-gnu/release/cast /usr/local/bin/cast
COPY --chmod=0744 --from=build /opt/foundry/target/x86_64-unknown-linux-gnu/release/anvil /usr/local/bin/anvil
COPY --chmod=0744 docker-entrypoint.sh /usr/bin/

STOPSIGNAL SIGQUIT

ENTRYPOINT ["/bin/sh", "-c"]


LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.name="Foundry" \
      org.label-schema.description="Foundry Anvil RPC" \
      org.label-schema.url="https://vcs.manifoldfinance.com/" \
      org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://github.com/gakonst/foundry.git" \
      org.label-schema.vendor="CommodityStream" \
      org.label-schema.version=$VERSION \
      org.label-schema.schema-version="1.0"
