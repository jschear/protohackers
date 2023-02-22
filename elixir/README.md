# Protohackers Elixir

Elixir umbrella project for protohackers solutions.

To create a new application:
```bash
cd apps/
mix new smoke_test --module SmokeTest --sup
```

Then copy over a `.dockerignore`, `Dockerfile`, and `fly.toml`. (Assuming we're deploying to fly.io using a docker image.)
