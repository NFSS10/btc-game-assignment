# BTC game assignment

Solution for the BTC game assignment.

<img width="441" height="357" alt="imagem" src="https://github.com/user-attachments/assets/17e7c83a-0998-4429-942a-cb6882118b61" />


## Table of Contents

- [Description](#description)
  - [System Architecture](#system-architecture)
  - [Frontend UI & Features](#frontend-ui--features)
  - [API Endpoints](#api-endpoints)
- [Development](#development)
  - [Installation](#installation)
  - [How to run](#how-to-run)
- [Demo deployment](#demo-deployment)


## Description

This project is a real-time Bitcoin price prediction game built with a Rust backend and a React (Vite) single-page application frontend. Players predict whether the market price of BTC/USD will go up or down over a 60-second window, earning or losing points based on market movements.

### System Architecture

The application is structured as a decoupled client-server architecture:
  - Backend (Rust): Built with Rust for high concurrency, low latency, and memory safety;
    - Live Price Feed: Connects to Binance via WebSockets to ingest real-time BTC/USD price ticks.
    - Fair play enforcement: As the server is the source of truth for all game state, it ensures that players cannot manipulate the game by submitting multiple guesses or tampering with the price feed. The server validates each guess against the current price and enforces a single-active-guess rule per player;
    - Resolution Engine: Evaluates active guesses on each price tick. A guess is resolved once at least 60 seconds have passed and a price change occurs;
    - Event Streaming (SSE): Utilizes a Server-Sent Events (`/game/events`) endpoint to stream deduplicated price ticks, score updates, and guess outcomes to connected clients in real time;
    - Player Management: Handles state initialization, returning user persistence, and single-active-guess validation;
    - Reliability: Implements robust error handling, reconnection logic and concurrency-safe state management to ensure correctness and responsiveness under high load.

  - Frontend (React + Vite): A responsive, single-page web app that provides a dynamic player experience driven entirely by SSE real-time updates.

#### Frontend UI & Features

The user interface automatically synchronizes with the SSE stream, updating state across all components without manual refreshes or periodic polling:
  - Score & Persistence: Displays current user score continuously and restores score history upon returning to the app.
  - Real-time Price Chart: Features a live line graph displaying historical and current BTC price movements, with the exact real-time price highlighted prominently in the center;
  - Action Controls: Provides "Up" and "Down" prediction buttons. The UI automatically locks prediction inputs while a guess is pending resolution to strictly enforce the single-active-guess rule;
  - Guesses Table: A live-updating history table below the action controls that tracks both past and pending guesses with the following details:
    - Time: Time at which the guess was placed;
    - Direction: The prediction made ("up" or "down");
    - Entry: BTC price value at the time the guess was placed;
    - Resolved: Price value at which the guess resolved (displays "Pending" until resolved);
    - Result: Outcome of the guess ("Win" or "Loss", or "Pending" while active).

#### API Endpoints
    POST /api/v1/players/init - Initializes a player session; restores existing state or registers a new player.
    GET /api/v1/players/:playerId/guesses - Fetches the list of all guesses (both resolved and pending) for a specific player.
    POST /api/v1/game/guess - Used to submit a new guess for a player, specifying the direction ("up" or "down").
    GET /api/v1/game/events?playerId=<playerId> - Establishes an SSE connection streaming live prices, player score updates, and resolved guess events.

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

<img width="1173" height="533" alt="AWS database" src="https://github.com/user-attachments/assets/f062950f-cb30-4ad6-bdba-72530fd2b4de" />


#### Client

The client is deployed to AWS Amplify. The client can be accessed via https://staging.d2ji01t3bj2zly.amplifyapp.com/

<img width="2356" height="930" alt="AWS Amplify deployement" src="https://github.com/user-attachments/assets/eef48ade-1b97-4ad7-a151-414a95ae991c" />

