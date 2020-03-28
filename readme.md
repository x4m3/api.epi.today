# api.epi.today

> api for epi.today

This is a rewrite of a web project (https://github.com/x4m3/epi.today) in the effort of separating the back-end from the front-end.

It's my first project in rust, so if you find bad code that could be rewritten don't hesitate to hit me up!

# development

Run `cargo build` to download and compile everything required for this project.

You can run the debug version of the project with `cargo run`.

The default listening port is `4242`.

# deployment

use docker

start application in container as daemon `docker run -d --restart unless-stopped --name api-epi-today -p 80:4242 x4m3/api-epi-today`. The container will automatically start when docker starts.

stop application `docker stop api-epi-today`