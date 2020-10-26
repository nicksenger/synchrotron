# Microbiome

Microbiome is a full stack dockerized Rust boilerplate. It includes a workspace containing a GRPC authentication service, API gateway, and WASM frontend.

## API Gateway

The API gateway & static file server is contained within the `api-gateway` package. It exposes a GraphQL API using [Juniper](https://github.com/graphql-rust/juniper) and [Actix Web](https://github.com/actix/actix-web).

## GRPC services

Protobufs and generated Rust code for GRPC service(s) are contained within the `schema` package. All services and the API gateway depend on `schema` to facilitate type-safe communication.

### Users service

The `users-service` package is an example GRPC microservice built using [Tonic](https://github.com/hyperium/tonic). It uses [SQLx](https://github.com/launchbadge/sqlx) to perform compile-time checked SQL queries.

## Frontend

The `frontend` package is a very simple example WASM application built using [Moxie](https://moxie.rs/). It uses Juniper's schema introspection in combination with [graphql-client](https://github.com/graphql-rust/graphql-client) to achieve type-safe communication with the API gateway, and manages state and side-effects using [moxie-streams](https://github.com/nicksenger/moxie-streams).
