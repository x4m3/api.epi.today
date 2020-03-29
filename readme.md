# api.epi.today

![Rust](https://github.com/x4m3/api.epi.today/workflows/Rust/badge.svg)
![Docker Hub](https://github.com/x4m3/api.epi.today/workflows/Docker%20Hub/badge.svg)

This is a rewrite of a my project [epi.today](https://github.com/x4m3/epi.today) in the effort of separating the back-end from the front-end.

It's my first project in rust, so if you find bad code that could be rewritten don't hesitate to open an issue!

## development

Run `cargo build` to compile the project.

You can run the debug version of the project with `cargo run`.

The listening port is `4242`.

⚠️ Warning: The server listens on **http** only, which means that **zero bytes** will be encrypted!  
There is confidential data that will be transferred between the client and the server, please keep security in mind when deploying.

## deployment

### binary

Run `cargo build --release` to compile the project ready to be released.

The binary will be available in `./target/release/api-epi-today`.

### docker

When building the [Dockerfile](Dockerfile), the server is compiled with the [musl library](https://github.com/emk/rust-musl-builder) and statically linked, in order to make the server portable and keeping the Docker image small.

The docker image is available on [Docker Hub](https://hub.docker.com/r/x4m3/api.epi.today) (please don't use the one in the GitHub registry).

Start the server in a container as a daemon with `docker run -d --restart unless-stopped --name api-epi-today x4m3/api-epi-today`.

Stop the container with `docker stop api-epi-today`.