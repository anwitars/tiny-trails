# Tiny Trails

Tiny Trails is a url shortener service that allows users to shorten long urls to a more manageable length. This service is built using Rust with PostgreSQL as the database.

## Why does this project exist?

The project has been created for educational and learning purposes only. It is not recommended to use this service in production as it lacks quality-of-life features and security measures.

Aside from all that (and the fact that it looks great on a resume), I want it to become more than a 'little side' project. The roadmap shows all the ambitious plans I have for this project.

## How is it different from other URL shorteners?

Although the features has not been implemented yet, my plan is to make it completely free, open-source and as transparent as possible.

Tiny Trails does not provide user accounts, but instead each Trail has a secret key whose owner can use to manage the trail. This avoids the need for user accounts (usernames, emails, passwords, etc.) and the need to store any personal information.

Information about each Trail can be viewed, such as the original URL the Trail points to and when was the Trail created. More detailed information is available to the owner of the Trail, such as the number of times the Trail has been accessed.

Dynamic Trails will be implemented as well, allowing the user to create trails that resolves to different URLs based on specific conditions. These specific Trails will be clearly marked as dynamic in the endpoint (`/dt/<trailid>`) and its info (`/info/<trailid>`). The info will also return the conditions that the dynamic trail has, while also marking which conditions have been met by the current request, and what it would resolve to.

## Roadmap

### Basic Features

- [x] Shorten URLs
- [x] Redirect to original URL
- [x] Trail statistics
  - [x] Total access count
  - [x] Unique access count
  - [x] History of the last week

### Authentication

- [x] Per-Trail authentication using autogenerated secrets (to avoid using accounts)
- [x] Delete Trails

### Telemetry

- [x] Trail access (hashed IP addresses)
- [ ] Access by country

### Security

- [x] Rate limiting for creating trails
- [x] Rate limiting for accessing trails

### Dynamic Trails

- [ ] Header-based conditions
- [ ] Datetime-based conditions
- [ ] IP-based conditions

### Features to rethink

- URLs should expire after a certain period of time not being accessed
  - Currently they expire after `x` hours (default is 1)

## About precautions

The service has no right to judge whether a URL is safe, malicious or anything else. It is the user's responsibility to decide whether to access a shortened URL or not. Although I stand by the fact that we should make short URLs as safe as possible, so in the future I plan to develop other piece of software (e.g. browser extension) to scan and show the original URL before accessing it (either via a middle-page, or directly in the browser where the short url is found).

## Telemetry

To provide a better service, Tiny Trails collects telemetry data. This includes:

- The number of times a trail has been accessed
  - Both unique and total

Trail access is counted by hashed IP addresses. This means that the service does not store any personal information about the user, but can determine if a user has accessed a trail before.

## Usage

### Building the project

To build the project, you need to have Rust installed. You can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

After installing Rust, you can build the project by running:

```sh
cargo build [--release]
```

And that is all it takes. The binary will be available at `./target/debug/tiny-trails` or `./target/release/tiny-trails`. To see what arguments you can pass to the binary, run:

```sh
tiny-trails --help
```

### Docker

The docker image is available at my [GitHub Container Registry](https://github.com/anwitars/tiny-trails/pkgs/container/tiny-trails). You can pull the image by running:

```sh
docker pull ghcr.io/anwitars/tiny-trails:latest
```

To run the image, you can use the following command:

```sh
docker run -d -p <port>:3000 ghcr.io/anwitars/tiny-trails:latest
```

Replace `<port>` with the port you want the service to be available at.
