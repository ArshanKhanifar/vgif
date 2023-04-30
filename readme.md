# vgif

`vgif` is a command-line program that allows you to extract segments of a video file and convert them to animated GIF files.

## Requirements

- Rust (version 1.51 or later)
- FFmpeg (version 4.0 or later)

## Installation

To install `vgif`, run the following command:

```
cargo install vgif
```


## Usage

To use `vgif`, run the following command:

```
vgif <path-to-video-file> [crop1, crop2, ...] --outfile=<output-file-name>
```

- `path-to-video-file`: The path to the input video file. Supported formats include MP4 and MOV.
- `[crop1, crop2, ...]`: A list of time ranges to extract from the input video file, in the format of `start_time:end_time`. The time ranges are separated by spaces. The start and end times can be specified in seconds, or in minutes and seconds (e.g. `1m10s:2m30s`).
- `--outfile=<output-file-name>`: The name of the output file. If not specified, the default name is `out.gif`.

For example, to extract two segments from a video file and save them as animated GIFs with default names, run the following command:


This will extract two segments from the `video.mp4` file (one from 1 minute 10 seconds to 2 minutes 30 seconds, and the other from 3 minutes to 4 minutes), and save them as `out_1.gif` and `out_2.gif` in a timestamped folder inside an `output` directory.

## Contributing

Contributions to `vgif` are welcome! If you find a bug or want to suggest a new feature, please open an issue or submit a pull request.

## License

`vgif` is licensed under the MIT license. See the `LICENSE` file for details.
