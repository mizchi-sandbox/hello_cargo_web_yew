# Hello cargo-web / stdweb / yew

## How to start

```sh
# setup
cargo install cargo-web
# TODO: Install guide about wasm32-unkwown-unkwown

# optional: emscripten for asm.js build
# TODO: write it later

# build
cargo web deploy # generate target/deploy

# start server with your static server

npm i -g http-server

http-server -p 10000 target/deploy
```

## LICENSE

MIT
