# BTC game assignment

Solution for the BTC game assignment.


## Table of Contents

- [Development](#development)
  - [Installation](#installation)
  - [How to run](#how-to-run)
- [Approach and Design Decisions](#approach-and-design-decisions)
- [Demo deployment](#demo-deployment)


## Development

### Installation

1. Set up the environment variables:

```bash
cp .env.template .env
```

### How to run

Simply run the following command to start the application:

```bash
docker compose -f ./compose.dev.yml up
```


## Demo deployment

Demo url: https://staging.d2ji01t3bj2zly.amplifyapp.com/

#### Server

I built the docker image and deployed to one of my servers. As per the assignment instructions, the server connects to a database in AWS, in this case, a PostgreSQL database. The server can be accessed via https://btc-game-server.nfss10.com/health

#### Client

The client is deployed to AWS Amplify. The client can be accessed via https://staging.d2ji01t3bj2zly.amplifyapp.com/
