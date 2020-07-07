# Wrust
a simple example of how to use websockets with Rust.

### Without Docker

to start the server:

```bash
$ cargo run --bin wrust-server
```

to start the client:

```bash
$ cargo run --bin wrust-client
```

Now you can go at `http://localhost:3000` and play with it.

### With Docker

to start the application:

```bash
$ docker-compose up
```

Wait until everything is done, then go at `http://localhost` and play with it.

*NOTE:* Depending on how you ports are being used, the port `80` might cause you issues. If so, just update the `docker-compose.yml` accordingly.
