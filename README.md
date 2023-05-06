# Task board

This is a tutorial project that implements a primitive task board.

It was created to learn the basic concepts of asynchronous programming in rust
using [Tokio](https://tokio.rs/), as well as to get acquainted with [hyper](https://hyper.rs/) and [next.js](https://nextjs.org/)

The project is based on the [tutorial from tokio](https://tokio.rs/tokio/tutorial).

The database or cache server is a minimal implementation of [Redis](https://redis.io/). The main application server is built using hyper. The frontend is built with next.js

# Run

#### Cash server
1. new terminal
2. `cd mini-casher`
3. `cargo run --bin cash-server`

#### CMD Cash server
1. new terminal
2. `cd mini-casher`
3. `cargo run --bin cmd`
4. check connection on terminal `ping` - you should get a response `pong`

#### App server
1. new terminal
2. `cd app-server`
3. `cargo run`

#### UI
1. new terminal
2. `cd static`
3. `npm run dev`
4. in browser `localhost:3000`
