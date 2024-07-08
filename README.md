# Mogcord
This is a basic messaging platform using Rust Axum.

## Prerequisites
1. Have Rust and cargo installed. (no clue what version)
2. Either have
   * MongolDB installed. (MongoDB v7.x.x)
   * Docker installed. (API v1.45)

## Getting Started
Add a .env file in project root.

```bash
#key for encoding jwt tokens
ACCES_TOKEN_KEY=
```


## How to run the server
```bash
#running on local machine (need to have MongolDB installed)
cargo run

#running on docker (need to have Docker installed)
docker compose up
```

## Me
<img src="https://i.imgur.com/qXyjT2u.jpg" width="400">
