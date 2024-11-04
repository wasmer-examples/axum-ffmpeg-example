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
$ wasmer run --net .
Listening on http://127.0.0.1:3000
```

Finally, you can send the file to the endpoint
```bash
$ curl -X POST -F "file=@small_test_video.mp4" http://localhost:3000/codecs
Input #0, mov,mp4,m4a,3gp,3g2,mj2, from '/tmp/small_test_video.mp4':
  Metadata:
    major_brand     : isom
    minor_version   : 512
    compatible_brands: isomiso2mp41
    encoder         : Lavf60.10.100
  Duration: 00:00:01.00, start: 0.000000, bitrate: 36 kb/s
  Stream #0:0[0x1](und): Video: mpeg4 (Simple Profile) (mp4v / 0x7634706D), yuv420p, 128x128 [SAR 1:1 DAR 1:1], 29 kb/s, 1 fps, 1 tbr, 16384 tbn (default)
    Metadata:
      handler_name    : VideoHandler
      vendor_id       : [0][0][0][0]
      encoder         : Lavc60.22.100 mpeg4
At least one output file must be specified
```

## Deploy on Wasmer Edge

The easiest way to deploy your Axum Rust app is to use the [Wasmer Edge](https://wasmer.io/products/edge).

```bash
wasmer deploy
```
