# Tiny Trails

Just another URL shortener service. Built for fun and learning purposes, and also to be used in portfolio.

## Why Python?

Most of my professional programming experience is in Python, and I had no public projects before. I though that is would be convenient to use Python for this project, so I can show my skills in it, and also because it is fast to develop in Python. My free time is limited, so I wanted to make something that I can finish in a reasonable time.

## Features

- Shorten URLs

And that's it. Do one thing and do it well. But other than just shortening them, it also provides very simple telemetrics to see how many times the shortened URL was accessed, both unique and total.

## Terminology

### Trail

A shortened URL. A [Token](#token) is generated for each Trail.

#### Trail Lifetime

A [Trail](#trail) is created with a lifetime provided by the user or defaulting to 3 days. The lifetime is the number of hours the Trail lives for from the time of it has been created. After the lifetime expires, the Trail can not be accessed anymore, not even with its [Token](#token).

#### Token

A unique identifier used to access restricted information and/or operations on a [Trail](#trail) by providing `X-Trail-Token` header in the request. All Trails have a unique Token, and can not be customized by the user.

### Visit

An access to a [Trail](#trail). A Visit is counted every time the Trail is accessed via the [traverse](#traverse-endpoint) endpoint. A Visit counts as unique if the [hashed IP address](#hashed-ip) of the visitor is not already recorded for the Trail.

### Peek

A Peek is a special kind of [Visit](#visit) that is counted when the [Trail](#trail) is accessed via the [peek](#peek-endpoint) endpoint. A Peek does not count as a Visit, and does not increase the visit count of the Trail. It is used to check if a Trail exists and/or to see the underlying URL of the Trail without being redirected to it.

### Hashed IP

A hashed version of the IP address of the visitor. It is used to count unique [Visits](#visit) to a [Trail](#trail). The hashing is done using SHA-256, and the resulting hash is stored in the database. This way, the original IP address is not stored, but it can still be used to check if a Visit is unique.

## Features

It shortens URLs, and that's it. It does one thing, but tries to be as transparent as possible, with as little telemetry as possible.

## Endpoints

The following endpoint documentations are simplified, and do not include all possible inputs, responses and errors. Please refer to the source code or the [OpenAPI documentation](https://github.com/anwitars/tiny-trails/blob/master/docs/openapi.json) for more details.

### /ping GET <a name="ping-endpoint"></a>

Check if the service is running. Returns a simple "pong" response.

### /pave POST <a name="pave-endpoint"></a>

Create a new [Trail](#trail) with a given URL and an optional lifetime.

### /t/{trail_id} GET <a name="traverse-endpoint-get"></a>

Traverse a [Trail](#trail) by its ID. This will redirect the user to the underlying URL of the Trail, and count a [Visit](#visit) for it.

### /t/{trail_id} DELETE <a name="traverse-endpoint-delete"></a>

Delete a [Trail](#trail) by its ID, making it inaccessible. Must also provide its [Token](#token) for authorization.

### /peek/{trail_id} GET <a name="peek-endpoint"></a>

Peek a [Trail](#trail) by its ID. This will return the underlying URL of the Trail in plain text format, and will not count a [Visit](#visit) for it.

### /info/{trail_id} GET <a name="info-endpoint"></a>

Get information about a [Trail](#trail) by its ID. For further information, see the [OpenAPI documentation](https://github.com/anwitars/tiny-trails/blob/master/docs/openapi.json)

## Installation

### Requirements

- Python 3.12 or higher (Probably the minimum version is 3.11 (due to `Self` type), but I have not tested it.)
- PostgreSQL (not sure about versions, but 15 and above works just fine)
- [Poetry](https://python-poetry.org/) for project and dependency management
- Docker (optional, for running the service in a container)

### Setup

1. Clone the repository
2. Install the dependencies using Poetry:
   ```bash
   poetry install
   ```

   if you wish to only install runtime dependencies, use:
   ```bash
   poetry install --without=dev,test
   ```
3. Familiarize yourself with the cli tool:
   ```bash
   tiny-trails --help
   ```
4. Serve the service:
   ```bash
   tiny-trails serve --host 0.0.0.0 --port 8000 --db postgresql://user:password@localhost:5432/tiny_trails
   ```

### Docker

You can also run the service in a Docker container.

#### Build the image

If you wish to build the image yourself, you can do so by running:
```bash
docker build -t tiny-trails . --load
```

And that's it. You can then run the image with:
```bash
docker run -p 8000:8000 tiny-trails --env TINY_TRAILS_DB=...
```

#### Run prebuilt image

If you wish to run the prebuilt image, you can do so by running:
```bash
docker run -p 8000:8000 ghcr.io/anwitars/tiny-trails:latest --env TINY_TRAILS_DB=...
```

## License

The project is issued under [MIT license](https://github.com/anwitars/tiny-trails/blob/master/LICENSE).
