![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Htmx](https://img.shields.io/badge/%3C/%3E%20htmx-3D72D7?style=for-the-badge&logo=mysl&logoColor=white)
![AlpineJs](https://img.shields.io/badge/Alpine%20JS-8BC0D0?style=for-the-badge&logo=alpinedotjs&logoColor=black)
![TailwindCSS](https://img.shields.io/badge/tailwindcss-%2338B2AC.svg?style=for-the-badge&logo=tailwind-css&logoColor=white)
![MongoDB](https://img.shields.io/badge/MongoDB-4EA94B?style=for-the-badge&logo=mongodb&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

[![Help Wanted](https://img.shields.io/github/issues/MogInc/Mogcord/help%20wanted?style=flat-square&color=%232EA043&label=help%20wanted)](https://github.com/MogInc/Mogcord/labels/help%20wanted)
[![Good First Issue](https://img.shields.io/github/issues/MogInc/Mogcord/good%20first%20issue?style=flat-square&color=%232EA043&label=good%20first%20issue)](https://github.com/MogInc/Mogcord/labels/good%20first%20issue)
[![Rust](https://github.com/MogInc/Mogcord/actions/workflows/rust.yml/badge.svg)](https://github.com/MogInc/Mogcord/actions/workflows/rust.yml)

# Mogcord
This is a basic messaging platform using Rust Axum.

## Prerequisites
* Have windows installed, if not modify the npm run script.
* Have Rust and cargo installed. (no clue what version)
1. For Local
    - Node.js (version doesn't matter, it's so you can use npm)
    - MongoDB (7.X.X)
2. For Docker
    - Docker (API v1.45)

## Getting Started
Add a .env file in project root.

```bash
#key for encoding jwt/acces tokens
ACCES_TOKEN_KEY=secret_pepper
```


## How to run the server
```bash
# 1a: run on local machine (need to have prerequisites installed)
npm run dev

# 1b: build and execute the binary
npm run debug
npm run release

# 1c: running on docker (need to have Docker installed)
docker compose up
```

## Me
<img src="https://i.imgur.com/qXyjT2u.jpg" width="400">
