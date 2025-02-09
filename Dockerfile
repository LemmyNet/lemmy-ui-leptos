FROM rust:slim-bookworm AS base
RUN apt update && apt -y install wget pkg-config libssl-dev
RUN wget -O- https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz | tar -xvz -C /usr/local/cargo/bin

FROM base AS leptos-ui
WORKDIR /usr/src/app

COPY *.toml Cargo.lock tailwind.config.js package.json pnpm-lock.yaml ./
COPY src src
COPY public public
COPY locales locales
COPY style style

RUN rustup target add wasm32-unknown-unknown
RUN cargo-binstall -y cargo-leptos
RUN wget -O- https://deb.nodesource.com/setup_20.x | bash
RUN apt-get install -y nodejs

# Enable corepack to use pnpm
RUN npm i -g corepack
RUN corepack enable
RUN pnpm install --frozen-lockfile

FROM leptos-ui AS playwright
COPY --from=leptos-ui . .
COPY end2end end2end
RUN pnpx playwright@1.44.1 install --with-deps
RUN cd end2end
RUN pnpm install --frozen-lockfile
RUN cd ..
ENV INTERNAL_HOST=lemmy:8536
ENV HTTPS=false
CMD cargo leptos end-to-end
