## Contributing

### Tools and environment

Install `cargo-leptos` with:

`cargo install cargo-leptos`

Install `taplo` with:

`cargo install taplo-cli`

Install `machete` with:

`cargo install cargo-machete`

Then run:

`pnpm i`

to install Tailwind and daisyUI.

You can run your own local instance of Lemmy or run the UI with a test instance provided by the Lemmy community.

Ensure that the version of the Lemmy API you are using in the UI is compatible with the instance you are using.

This project does yet not handle multiple versions of the Lemmy API gracefully. Changing the API version will cause compilation errors in this project and errors when communicating with your Lemmy instance.

Create the environment variables to point to your instance and specify Tailwind version (defaults shown here):

```
export LEPTOS_TAILWIND_VERSION=v3.4.1
export LEMMY_UI_LEPTOS_LEMMY_HOST=localhost:8536
export LEMMY_UI_LEPTOS_LEMMY_HTTPS=false
```

Compile and run with:

`cargo leptos watch`

and browse to `http://localhost:1237` to see the UI.

Any changes you make while coding might require a page refresh as the automatic reload may become detached.

### Formatting

Code submissions need to follow strict formatting guidelines. Run `./format.sh` or use the commands within to automate this process.
