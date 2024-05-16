FROM rust:slim-bookworm AS base
RUN apt update && apt -y install libssl-dev pkg-config build-essential gcc make wget
RUN wget -O- https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz | tar -xvz -C /usr/local/cargo/bin

FROM base AS leptos-ui
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo-binstall -y cargo-leptos
RUN wget -O- https://deb.nodesource.com/setup_20.x | bash
RUN apt-get install -y nodejs
RUN npm install -g pnpm
RUN pnpm install --frozen-lockfile

FROM leptos-ui AS playwright
COPY . .
RUN pnpx playwright@1.44.0 install --with-deps
RUN cd end2end
RUN pnpm install --frozen-lockfile
RUN cd ..
ENV INTERNAL_HOST=lemmy:8536
ENV HTTPS=false