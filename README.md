This is an [Axum](https://github.com/tokio-rs/axum) Web Server that uses ffmpeg to return the codec information of a file.

## Getting started

First, build the project using [`cargo-wasix`](https://crates.io/crates/cargo-wasix):

```bash
$ cargo wasix build --release
```

Then you can generate a small test file using ffmpeg:
```bash
$ wasmer run wasmer/ffmpeg --dir=. -- -f lavfi -i testsrc=duration=1:size=128x128:rate=1 small_test_video.mp4
```

Then, you can run the server easily using Wasmer:

```bash
$ wasmer run . --env PORT=8080
Listening on http://127.0.0.1:8080
```

Finally, you can send the file to the endpoint
```bash
curl -X POST -F "file=@small_test_video.mp4" http://localhost:8080/codecs
```

## Deploy on Wasmer Edge

The easiest way to deploy your Axum Rust app is to use the [Wasmer Edge](https://wasmer.io/products/edge).

Live example: https://wasix-axum-example.wasmer.app

```bash
wasmer deploy
```
