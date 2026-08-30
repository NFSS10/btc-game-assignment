# BTC game server


## Table of Contents

- [Requirements](#requirements)
- [Development](#development)
  - [Installation](#installation)
  - [How to run](#how-to-run)
  - [Environment variables](#environment-variables)
  - [Other commands](#other-commands)
- [Production](#production)


## Requirements
- Rust 1.98.x
- Docker + Docker Compose


## Development

### Installation

1. Install helper tools:

```bash
cargo install cargo-make cargo-watch
```

2. Run setup command:

```bash
cargo make setup
```

3. Set up environment variables:

```bash
cp .env.template .env
```

### How to run

1. In one terminal, run the following command to start the services:

```bash
cargo make services:up
```

2. In another terminal, run the following command to start the server:

```bash
cargo make dev:run
```

<details>
  <summary>Alternatively, use the development Docker image</summary>

  ```bash
  docker build -f ./dev.Dockerfile -t btc-game-server:dev ./
  docker run --rm -it --init \
    --network host \
    -v ./:/app/server \
    -v btc_game_server_target_cache:/app/server/target \
    -v btc_game_server_cargo_registry_cache:/usr/local/cargo/registry \
    btc-game-server:dev
  ```

  This development image is configured for a seamless development experience, including hot reload.
</details>

### Environment variables

| Argument        | Required           | Description                                |
| --------------- | ------------------ | ------------------------------------------ |
| `PORT`          | :x:                | The port the server runs on                |
| `DATABASE_URL`  | :white_check_mark: | PostgreSQL database URI                    |
| `CRYPTO_SYMBOL` | :x:                | Cryptocurrency symbol (default: "BTCUSDT") |


### Other commands

Check [`Makefile.toml`](./Makefile.toml) for all available commands.


## Production

1. Simply run the production compose file:

```bash
docker compose -f ./compose.prod.yml up
```

> [!NOTE]  
> Running the compose command will handle building the production image and running it.
> 
> You can also build the production image manually:
> ```bash
> docker build -f ./prod.Dockerfile -t btc-game-server:prod ./
> ```
