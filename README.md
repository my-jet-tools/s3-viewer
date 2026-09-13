# s3-viewer

A small browser-based viewer for S3-compatible storage. It is read-only.

The screen has two parts:

- **Left: a folder tree.** The root level is the buckets configured in the settings file.
  - Double-click a bucket or folder to load the level below it: its folders and files (S3 delimiter `/`). Levels load lazily, one at a time.
  - Double-click a file to download it.
- **Right: details.** A table of the selected folder, or a card for the selected file with a Download button.

A single process serves both the REST API and the UI. The UI is a Dioxus client-side (wasm) app, built into `wwwroot/`.

## Layout

| Path               | What it is |
|--------------------|------------|
| `Cargo.toml`, `src/` | The API server: `my-http-server` with controllers, Swagger and `StaticFilesMiddleware`. |
| `rest-api-shared/` | Wire contract: route constants plus request and response models. The server uses it with the `server` feature; the UI uses it without features. |
| `ui/`              | The Dioxus 0.7 client-side app. HTTP goes through FlUrl. |
| `wwwroot/`         | The built UI (output of `./build-ui.sh`), served by the API server. |
| `build-ui.sh`      | Wrapper around `ui/build.sh`. |
| `settings.example.yaml` | Settings file format. |
| `Dockerfile`       | Packages the release binary together with `wwwroot/`. |

## API

All three routes are `GET`. The Swagger UI is at `/swagger`.

| Route | Query | Result |
|-------|-------|--------|
| `/api/buckets/v1/list` | none | `ListBucketsResponse`: the buckets from settings, in settings order. |
| `/api/objects/v1/list` | `bucket`, `prefix` (optional; empty or absent means the bucket root) | `ListObjectsResponse`: the folders and files directly under `prefix`. `is_truncated` is `true` when the server stopped paging (100 × 1000 keys). |
| `/api/objects/v1/download` | `bucket`, `key` | The object streamed as `Content-Disposition: attachment`. |

Error codes:
- **404:** the bucket is not in settings, or the bucket or key does not exist in S3.
- **500:** any other S3 failure. The client gets a short message and the details go to the log.

## Settings

The settings file is read at start-up from:

1. the path in the `S3_VIEWER_SETTINGS` environment variable, if set, or
2. `~/.s3-viewer`.

The format is in `settings.example.yaml`:

```yaml
http_port: 8000          # optional, 8000 when absent
buckets:                 # the root of the tree, in this order; names must be unique
  - name: my-bucket
    endpoint: https://nbg1.your-objectstorage.com
    region: nbg1
    access_key: YOUR_ACCESS_KEY
    secret_key: YOUR_SECRET_KEY
```

Credentials stay on the server; the UI only ever sees bucket names. A missing or invalid file stops start-up with a message that says what to fix.

## Build the UI

This needs the wasm target and the Dioxus CLI **0.7.10**. The CLI version must match the `dioxus` crate version in `ui/Cargo.toml`.

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.10 --locked

./build-ui.sh
```

`ui/build.sh` does three things:
1. Runs `dx build --release --web`.
2. Replaces `wwwroot/` with the output.
3. Starts from a clean dx output dir, so old hashed js/wasm files are not carried over.

Run it after every change under `ui/`, and commit `wwwroot/` together with the change.

## Run

Start it **from the repo root**: the server serves `./wwwroot` relative to the working directory.

```sh
cargo run --release
# s3-viewer 0.1.0 is listening at http://0.0.0.0:8000
```

Then open <http://localhost:8000> (or whatever `http_port` is set to).

`StaticFilesMiddleware` answers any GET path that is not an API route or `/swagger` with `wwwroot/index.html`, so the client-side router owns every other path. A mistyped `/api/...` URL therefore returns `index.html`, not a 404.

### Docker

```sh
cargo build --release && ./build-ui.sh
docker build -t s3-viewer .
docker run -p 8000:8000 -v ~/.s3-viewer:/root/.s3-viewer:ro s3-viewer
```

## Dependency note: my-s3

The server needs `my-s3` with `S3Client::list_objects_v2` and `S3Client::download_file_as_stream` (plus `S3ListObjectsRequest`, `S3ListObjectsPage`, `S3ObjectInfo` and `S3DownloadStream`).

This API was planned as tag `0.1.1`, but it was released by **moving tag `0.1.0`** to commit `8de9f25e`, and `Cargo.toml` pins `tag = "0.1.0"`. The old `0.1.0` (`a06384a`) does not have these methods.

If you get errors like `no method named list_objects_v2`, a local cargo cache or `Cargo.lock` is still on the old commit. Run:

```sh
cargo update -p my-s3
```
