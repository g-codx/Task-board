# Cash server

A project for learning [Tokio](https://tokio.rs/tokio/tutorial) and [Redis](https://redis.io/).
A simpler version of the [mini-redis](https://github.com/tokio-rs/mini-redis)

You can run the server

    cargo run --bin cash-server

And console

    cargo run --bin cmd

Available commands in the console
- `get 'key'` - get value by key
- `set 'key' 'value'` - set a new value
- `len` - map length
- `all` - load all entity
- `delete 'key'` - delete by key