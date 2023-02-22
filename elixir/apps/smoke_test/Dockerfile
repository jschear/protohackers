FROM elixir:latest as build
COPY . .
RUN MIX_ENV=prod mix release --path release/

FROM elixir:latest as prod
WORKDIR /app
COPY --from=build /release/ .
EXPOSE 8080
ENV PORT=8080
ENTRYPOINT ["bin/smoke_test"]
CMD ["start"]
