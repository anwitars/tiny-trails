# Trails

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/anwitars/tiny-trails/blob/master/LICENSE)
![Python Version](https://img.shields.io/badge/Python-3.12+-blue)
![Docker](https://img.shields.io/badge/Docker-available-blue)
[![API Docs](https://img.shields.io/badge/API%20Docs-online-brightgreen)](https://api.trls.link/docs)

**A transparent URL shortener API with no accounts, no tracking, and no paywalls.** Public API available at [api.trls.link](https://api.trls.link/docs).

## 🚀 Quick Start

Shorten a URL:

```sh
curl -X POST https://api.trls.link/pave -H "Content-Type: application/json" \
  -d '{"url": "https://google.com"}'
```

You will receive a JSON response containing a `trail_id`.
Open it in your browser: `https://api.trls.link/t/{trail_id}`
You will be redirected to the original URL — and a [visit](#visit) will be counted.

## ✨ Features

It shortens URLs. That's it — but with a twist:
- **No accounts, no tiers** — everything is available for free, no sign-up required.
- **Peek before you click (if you wish to)** — see the destination URL without being redirected first.
- **Privacy first** — no IP storage, only hashed values for visit counting.
- **Simple, predictable API** — easy to integrate everywhere.

## 🐍 Why Python?

- Fast to develop and iterate on.
- I know it best — fewer bugs, faster features.
- If Trails grows, I might rewrite it in a more performant language.

## 📖 Terminology

### Trail

A shortened URL. Each Trail has its own [Token](#token).

#### Trail Lifetime

Set by the user or defaulting to 3 days. After expiry, the [Trail](#trail) will be inaccessible, even with its [Token](#token).

#### Token

Unique secret key for managing a [Trail](#trail). Sent via `X-Trail-Token` header.

### Visit

A redirect through a [Trail's](#trail) [traverse](#traverse-endpoint-get) endpoint. Counts as **unique** if the hashed IP has not visited before.

### Peek

View the original URL without redirecting. Does not count as a [Visit](#visit).

### Hashed IP

SHA-256 hash of the visitor’s IP, stored only for uniqueness checks.

## 📡 Endpoints

*This is a quick reference. Full details in [OpenAPI Spec](https://github.com/anwitars/tiny-trails/blob/master/docs/openapi.json)*

### /ping GET <a name="ping-endpoint"></a>

Check service status.

```sh
curl https://api.trls.link/ping

# pong
```

### /pave POST <a name="pave-endpoint"></a>

Create a new [Trail](#trail).

```sh
curl -X POST https://api.trls.link/pave -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'

# {"trail_id": "abc123", "url": "https://example.com", "token": "xyz789"}
```

### /t/{trail_id} GET <a name="traverse-endpoint-get"></a>

Redirect to original URL and count a [Visit](#visit).

```sh
curl -i https://api.trls.link/t/abc123

# HTTP/1.1 302 Found
# Location: https://example.com
```

### /t/{trail_id} DELETE <a name="traverse-endpoint-delete"></a>

Delete a [Trail](#trail) (requires [Token](#token)).

```sh
curl -X DELETE https://api.trls.link/t/abc123 -H "X-Trail-Token: xyz789"
```

### /peek/{trail_id} GET <a name="peek-endpoint"></a>

Reveal the original URL without visiting.

```sh
curl https://api.trls.link/peek/abc123

# https://example.com
```

### /info/{trail_id} GET <a name="info-endpoint"></a>

Get [Trail](#trail) info.

```sh
curl https://api.trls.link/info/abc123

# {
#    "id": "abc123",
#    "url": "https://example.com",
#    "visits": {
#       "all": 10,
#       "unique": 8,
#    },
#    "created": "2023-10-01T12:00:00Z",
#    "lifetime": 72
# }
```

## 🛠 Installation

### Requirements

- **Python 3.12+** (Likely works on 3.11+)
- **PostgreSQL 15+**
- [Poetry](https://python-poetry.org/) for project and dependency management
- Docker (optional, for self-hosting)

### Local Setup

```sh
git clone https://github.com/anwitars/tiny-trails
cd tiny-trails
poetry install
tiny-trails serve --host 0.0.0.0 --port 8000 \
  --db postgresql://user:pass@localhost:5432/tiny_trails
```

Install runtime only dependencies:
```sh
poetry install --without=dev,test
```

CLI help:
```sh
tiny-trails --help
```

### 🐳 Docker

You can also run the service in a Docker container.

#### Build the image

Build:
```sh
docker build -t tiny-trails . --load
docker run -p 8000:8000 -e TINY_TRAILS_DB=... tiny-trails
```

Run Prebuilt:

```bash
docker run -p 8000:8000 -e TINY_TRAILS_DB=... \
  ghcr.io/anwitars/tiny-trails:latest
```

## 📜 License

The project is issued under [MIT license](https://github.com/anwitars/tiny-trails/blob/master/LICENSE).
