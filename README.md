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
| `wwwroot/`         | The built UI (output of `./build-ui.sh`), served by the API server. **Committed**: CI does not build the UI. |
| `build-ui.sh`      | Wrapper around `ui/build.sh`. |
| `settings.example.yaml` | Settings file format. |
| `build.rs`         | Runs `ci-utils` on every `cargo build` to generate the next two rows. |
| `Dockerfile`       | **Generated** by `build.rs`. Packages the release binary together with `wwwroot/`. |
| `.github/workflows/` | **Generated** by `build.rs`: `release.yaml` (tag → Docker image) and `test.yml` (build + tests on every push). |

## API

All three routes are `GET`. The Swagger UI is at `/swagger`.

| Route | Query | Result |
|-------|-------|--------|
| `/api/buckets/v1/list` | none | `ListBucketsResponse`: the buckets from settings, in settings order. |
| `/api/objects/v1/list` | `bucket`, `prefix` (optional; empty or absent means the bucket root) | `ListObjectsResponse`: the folders and files directly under `prefix`. `is_truncated` is `true` when the server stopped paging (100 × 1000 keys). |
| `/api/objects/v1/download` | `bucket`, `key` | The object streamed as `Content-Disposition: attachment`. |

Error codes:
- **404:** the bucket is not in settings, or the bucket or key does not exist in S3.
- **404 `API route not found`:** the path is under `/api` but no route answers it (see [Unknown `/api` routes](#unknown-api-routes)).
- **500:** any other S3 failure. The client gets a short message and the details go to the log.

### Unknown `/api` routes

The middlewares run in this order: Swagger, the API controllers, `ApiRouteNotFoundMiddleware`, `StaticFilesMiddleware`.

`ApiRouteNotFoundMiddleware` (`src/http_server/api_route_not_found_middleware.rs`) answers **404** with the text `API route not found` for any request whose first path segment is `api`, compared case-insensitively. It does this for every HTTP method. Real API routes never reach it, because the controllers answer them first. So a mistyped `/api/...` URL, or a known route called with the wrong method, fails loudly. Without this middleware it would get `index.html` with 200.

Only the whole first segment counts:

| Path | Result |
|------|--------|
| `/api`, `/api/`, `/api/typo`, `/API/buckets/v1/nope` | 404 `API route not found` |
| `/apis`, `/api-docs`, `/assets/api`, `/buckets/api/list` | not under `/api`; handled by the static files as usual |

The 404 is not written to the log, because anyone can generate unknown URLs.

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

Run it after every change under `ui/`, and commit `wwwroot/` together with the change. The release workflow ships whatever `wwwroot/` is committed.

## Run

Start it **from the repo root**: the server serves `./wwwroot` relative to the working directory.

```sh
cargo run --release
# s3-viewer 0.1.0 is listening at http://0.0.0.0:8000
```

Then open <http://localhost:8000> (or whatever `http_port` is set to).

`StaticFilesMiddleware` answers any GET path that is not an API route or `/swagger` with `wwwroot/index.html`, so the client-side router owns every other path. The exception is `/api`: an unknown path under it gets a 404 (see [Unknown `/api` routes](#unknown-api-routes)).

### Docker

```sh
cargo build --release          # also regenerates the Dockerfile
docker build -t s3-viewer .
docker run -p 8000:8000 -v ~/.s3-viewer:/root/.s3-viewer:ro s3-viewer
```

The image is `ubuntu:22.04` and has no `WORKDIR`, so the process runs in `/`. The binary is `/target/release/s3-viewer`, `./wwwroot` resolves to `/wwwroot`, and `~/.s3-viewer` is `/root/.s3-viewer`.

## Lints

All lint levels are in `Cargo.toml`, not in the sources:

```toml
[lints.rust]
warnings = "deny"

[lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
result_large_err = "allow"   # the one framework-dictated exception, see below
```

`src/` has no `#![deny]` and no `#[allow]` attributes. A source attribute overrides the manifest: a crate-level `#![deny(clippy::all)]` would re-deny the exception, and a local `#[allow]` hides a lint where nobody looks. Before a change is done, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` must both pass.

**The one exception: `result_large_err`.** `http_route` makes every action's `handle_request` return `Result<HttpOkResult, HttpFailResult>` as is. `HttpFailResult` is `my-http-server`'s type and is larger than clippy's 128-byte limit, so it can't be boxed on our side. With the lint set to `warn`, the only hits are the `handle_request` fns of `ListObjectsAction` and `DownloadObjectAction`. Everything below the handlers returns the small `S3ViewerError`, so the exception hides nothing of ours. Delete it when `HttpFailResult` gets smaller upstream.

**`unused_async` is not an exception.** `ListBucketsAction` has nothing to await, so its `handle_request` is a plain fn that returns `std::future::ready(...)`. `http_route` only needs something it can `.await`, not an `async fn`.

## CI and release

`build.rs` runs `CiGenerator` from `ci-utils` (tag `0.1.3`) on every `cargo build` and rewrites:

- `Dockerfile`: `ubuntu:22.04`, the binary at `./target/release/s3-viewer`, and `./wwwroot` copied next to it.
- `.github/workflows/release.yaml`: runs on any tag. It sets the `Cargo.toml` version to the tag, runs `cargo build --release`, builds the image and pushes `ghcr.io/${{ github.repository }}:<tag>`.
- `.github/workflows/test.yml`: runs `cargo build --all-features` and `cargo test` on every push and pull request.

**Do not edit these files by hand.** The next `cargo build` overwrites them. Change `build.rs` instead, and commit what it generates.

**The release builds only the Rust server.** Nothing in CI builds the UI, so the image contains whatever `wwwroot/` is committed.

### Releasing

1. If anything under `ui/` changed since the last release, run `./build-ui.sh` and **commit `wwwroot/`**.
2. Run `cargo build && cargo clippy --all-targets -- -D warnings && cargo test`. Commit and push, including `Dockerfile` and `.github/`.
3. Create the release. The tag is the bare version; the workflow writes it into `Cargo.toml` and uses it as the image tag:
   ```sh
   gh release create 0.1.0 --title "0.1.0" --notes ""
   ```
4. Follow the build:
   ```sh
   gh run list --limit 5
   gh run watch <run-id>
   gh run view <run-id> --log-failed
   ```
   The result is the image `ghcr.io/my-jet-tools/s3-viewer:0.1.0`.

To re-deploy the same version:

```sh
gh release delete 0.1.0 --yes --cleanup-tag
gh release create 0.1.0 --title "0.1.0" --notes ""
```

### Before the first release

- **The GitHub repository does not exist yet.** `origin` points at `git@github.com:my-jet-tools/s3-viewer.git`, but the repository has to be created first.
- Add the `PUBLISH_TOKEN` repository secret (Settings → Secrets and variables → Actions), with a token that can push packages to ghcr.io. The release workflow uses it for the build and for `docker login`.
- Push the workflow files before the first tag. GitHub does not run a workflow that was not in the repository when the tag was created.
- The image name comes from `github.repository`, and a Docker image name must be lowercase. That works for the `my-jet-tools` owner. If the repository ends up under an owner with capitals (for example `MyJetTools`), `docker build -t` fails. In that case set a lowercase name in `build.rs` with `set_docker_image_name` (see `resolve_image_name` in `cargo-cache/build.rs`).

## Dependency note: my-s3

The server needs `my-s3` with `S3Client::list_objects_v2` and `S3Client::download_file_as_stream` (plus `S3ListObjectsRequest`, `S3ListObjectsPage`, `S3ObjectInfo` and `S3DownloadStream`).

This API was planned as tag `0.1.1`, but it was released by **moving tag `0.1.0`** to commit `8de9f25e`, and `Cargo.toml` pins `tag = "0.1.0"`. The old `0.1.0` (`a06384a`) does not have these methods.

If you get errors like `no method named list_objects_v2`, a local cargo cache or `Cargo.lock` is still on the old commit. Run:

```sh
cargo update -p my-s3
```
