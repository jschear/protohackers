FROM elixir:alpine as builder
ARG APP
RUN test -n "$APP" || (echo "APP not set" && false)
COPY . .
RUN MIX_ENV=prod mix release ${APP}

FROM elixir:alpine as prod
ARG APP
WORKDIR /app
COPY --from=builder /_build/prod/rel/${APP} .
ENV PORT=8080
EXPOSE ${PORT}
RUN ln -s /app/bin/${APP} /app/bin/entrypoint
ENTRYPOINT ["/app/bin/entrypoint"]
CMD ["start"]
