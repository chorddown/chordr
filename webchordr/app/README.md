# Chorddown Web

> The web UI and heart of the Chorddown project

[Trunk](https://trunk-rs.github.io/trunk/) is used to build the project.

## Build & deploy

1. `cd webchordr/app/`
2. Build a catalog from your song files: `cargo run -p chordr --release -- build-catalog static/songs/ static/catalog.json`
3. Build the web-app using `trunk build --release`
4. Copy the contents of `dist/` to the server's document root

> Tip: Check out `build-deploy.sh` in the project root

## Development

1. `cd webchordr/app/`
2. Build a catalog from your song files (see above)
3. Start the development server `trunk serve`
