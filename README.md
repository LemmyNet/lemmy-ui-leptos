# Lemmy-UI-Leptos

A complete rewrite of [Lemmy UI](//github.com/LemmyNet/lemmy-ui) in [Rust](//www.rust-lang.org/), [Leptos](//github.com/leptos-rs/leptos), [Daisy](//daisyui.com) and [Tailwind](//tailwindcss.com).

Using Rust everywhere means we get to use Rust's expressive type system and powerful language in the UI. It also means we inherit types and APIs from the server project [Lemmy](//github.com/LemmyNet/lemmy) that automates consistency and enables isomorphic code-reuse between components.

Leptos's signal based framework is both fast and easy to use making it ideal for apps based on web technologies.

Daisy and Tailwind give us components and utility classes that look great and are compiled into the project efficiently.

## Development

See [CONTRIBUTING.md](/CONTRIBUTING.md) for information on setting up your development environment.

It's a standard contemporary web development environment. The development feedback loop is made fast and convenient with the [cargo-leptos](//github.com/leptos-rs/cargo-leptos) CLI development server.

## Objectives

- initially leverage pure Daisy components to provide common component styling with the least markup
- when a custom look and feel is agreed upon, implement using Tailwind's fine grained styling
- use Tailwind's layout and responsive tools to adapt UI to screens of all common sizes
- use isomorphic Leptos code to ensure that features work in the following contexts:
  - SSR only - server side rendering only. Search engine bots and browsers with diverse technical requirements (JS and WASM are disabled) must be able to read and interact with all core features. There will be sophisticated (non-core) features where we agree this is not possible
  - Hydrate - features progressively enhance from being rendered on the server to running almost entirely in the browser (JS and WASM are available). Feature logic must handle this context switch gracefully
  - CSR only - client side rendering only - when a mobile/desktop app framework target is agreed upon (e.g. Tauri) all UI and interaction code is bundled into an app that communicates directly with its Lemmy instance
- all core features should be accessible to as diverse a user base as we agree is possible
- all UI text must be internationalized and rendered effectively for RTL languages
- the badge feature must be recognizable across all Lemmy front ends for ease of identification and administration
