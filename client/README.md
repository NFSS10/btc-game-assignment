# BTC game client


## Table of Contents

- [Requirements](#requirements)
- [Development](#development)
  - [Installation](#installation)
  - [How to run](#how-to-run)
  - [Environment variables](#environment-variables)
  - [Other commands](#other-commands)


## Requirements
- pnpm 11.9.x


## Development

### Installation

1. Install the dependencies:

```bash
pnpm install
```

2. Set up environment variables:

```bash
cp .env.template .env
```

### How to run

1. Simply run the following command to start the development server:

```bash
pnpm dev
```

<details>
  <summary>Alternatively, use the development Docker image</summary>

  ```bash
  docker build -f ./dev.Dockerfile -t btc-game-client:dev ./
  docker run --rm -it \
    -p 3000:3000 \
    -v ./:/app/client \
    -v btc_client_dev_node_modules_cache:/app/client/node_modules \
    btc-game-client:dev
  ```

  This development image is configured for a seamless development experience, including hot reload.
</details>

### Environment variables

| Argument              | Required           | Description                           |
| --------------------- | ------------------ | ------------------------------------- |
| `VITE_SERVER_API_URL` | :white_check_mark: | The URL that points to the server API |

### Other commands

Prettify + lint the code:

```bash
pnpm pretty
```

Run the tests:

```bash
pnpm test
```

Build the project:

```bash
pnpm build
```
