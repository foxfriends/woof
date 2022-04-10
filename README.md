# Rust Rest Framework

An attempt to make building a simple CRUD API in Rust as mindless as Django Rest
Framework makes it for Django apps.

> 🚧 Work in progress. The end goal of this is to be a library that makes it possible
> to derive the equivalent of a DRF `ModelViewSet` for a Rust struct.
>
> For now, this is just an exploration into what the output of that derive might be.

## Roadmap

This is just an estimate of things that might someday happen:

- [ ] Build the output manually once to see what it might be like
- [ ] Abstract common functionality into traits backed by `actix-web` and `sea-query` traits
- [ ] Build derive macros that will derive all the required traits from a struct
- [ ] Further abstract the database backend, so we are not tied to `sea-query`
- [ ] Further abstract the web frontend, so we are not tied to `actix-web`
