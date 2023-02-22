# Protohackers Elixir

Elixir umbrella project for protohackers solutions.

To create a new application:
```bash
cd apps/
mix new smoke_test --module SmokeTest --sup
```
Then add a new entry in `mix.exs`'s `releases` block.

To build an image:
```bash
docker build . --build-arg=APP=smoke_test
```

To deploy to fly.io:
```bash
fly deploy --build-arg=APP=smoke_test
```
