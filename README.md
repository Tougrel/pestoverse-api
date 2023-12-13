# Pestoverse API

[![License](https://shieldio.tougrel.dev/github/license/Tougrel/pestoverse-api?style=for-the-badge)](https://github.com/Tougrel/pestoverse-api/blob/main/LICENSE)
[![GitHub Contributors](https://shieldio.tougrel.dev/github/contributors/Tougrel/pestoverse-api?style=for-the-badge)](https://github.com/Tougrel/pestoverse-api/graphs/contributors)
[![GitHub Issues](https://shieldio.tougrel.dev/github/issues/Tougrel/pestoverse-api?style=for-the-badge)](https://github.com/Tougrel/pestoverse-api/issues)
[![GitHub pull requests](https://shieldio.tougrel.dev/github/issues-pr/Tougrel/pestoverse-api?style=for-the-badge)](https://github.com/Tougrel/pestoverse-api/pulls)
![Static Badge](https://shieldio.tougrel.dev/badge/Powered_by_Cloudflare-F38020?style=for-the-badge&logo=Cloudflare&logoColor=white)

## Wrangler

Wrangler is used to develop, deploy, and configure your Worker via CLI.

Further documentation for Wrangler can be found [here](https://developers.cloudflare.com/workers/tooling/wrangler).

## Usage

This template starts you off with a `src/lib.rs` file, acting as an entrypoint for requests hitting your Worker. Feel free to add more code in this file, or create Rust modules anywhere else for this project to use.

With `wrangler`, you can build, test, and deploy your Worker with the following commands:

```sh
# run your Worker in an ideal development workflow (with a local server, file watcher & more)
$ npm run dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
$ npm run deploy
```

Read the latest `worker` crate documentation here: https://docs.rs/worker

## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is meant to be executed as compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the [`workers-rs`](https://github.com/cloudflare/workers-rs) project README.
