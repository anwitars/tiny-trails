# Tiny Trails

Tiny Trails is a url shortener service that allows users to shorten long urls to a more manageable length. This service is built using Rust and currently uses SQLite as the database.

## Why does this project exist?

The project has been created for educational and learning purposes only. It is not recommended to use this service in production as it lacks quality-of-life features and security measures.

Aside from all that (and the fact that it looks great on a resume), I want it to become more than a 'little side' project. The roadmap shows all the ambitious plans I have for this project.

## How is it different from other URL shorteners?

Although the features has not been implemented yet, my plan is to make it completely free, open-source and as transparent as possible.

Tiny Trails does not provide user accounts, but instead each Trail has a secret key whose owner can use to manage the trail. This avoids the need for user accounts (usernames, emails, passwords, etc.) and the need to store any personal information.

Information about each Trail can be viewed, such as the original URL the Trail points to and when was the Trail created. More detailed information is available to the owner of the Trail, such as the number of times the Trail has been accessed.

## Roadmap

### Basic Features

- [x] Shorten URLs
- [x] Redirect to original URL
- [x] View Trail information (with limited statistics)

### Authentication

- [ ] Per-Trail authentication using autogenerated secrets (to avoid using accounts)
- [ ] Delete Trails
- [ ] Trail statistics

### Telemetry

- [x] Trail access (hashed IP addresses)
- [ ] Access by country

### Features to rethink

- URLs should expire after a certain period of time not being accessed
  - Currently they expire after `x` hours (default is 1)

## Telemetry

To provide a better service, Tiny Trails collects telemetry data. This includes:

- The number of times a trail has been accessed
  - Both unique and total

Trail access is counted by hashed IP addresses. This means that the service does not store any personal information about the user, but can determine if a user has accessed a trail before.
