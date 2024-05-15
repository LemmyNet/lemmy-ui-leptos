FROM rust:slim-bookworm AS base
RUN apt update && apt -y install libssl-dev pkg-config build-essential gcc make wget
RUN wget -O- https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz | tar -xvz -C /usr/local/cargo/bin

FROM base AS leptos-ui.base
RUN rustup target add wasm32-unknown-unknown
RUN cargo-binstall -y cargo-leptos
RUN wget -O- https://deb.nodesource.com/setup_20.x | bash
RUN apt-get install -y nodejs
RUN npm install -g pnpm

FROM leptos-ui.base AS playwright.base
RUN pnpx playwright@1.44.0 install --with-deps
ENV INTERNAL_HOST=lemmy:8536
ENV HTTPS=false

# When running in CI, no need to copy project files to image since woodpecker already does that for us
FROM leptos-ui.base AS leptos-ui.ci
RUN pnpm install --frozen-lockfile

FROM playwright.base AS playwright.ci
RUN cd end2end
RUN pnpm install --frozen-lockfile
RUN cd ..

# Need to manually copy project files to image when running from script, e.g. scripts/run_end2end_tests.sh
FROM leptos-ui.base AS leptos-ui.script
COPY . .
RUN pnpm install --frozen-lockfile

FROM playwright.base AS playwright.script
COPY . .
RUN cd end2end
RUN pnpm install --frozen-lockfile
RUN cd ..