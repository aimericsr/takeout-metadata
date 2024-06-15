[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/takeout-metadata
[crates-url]: https://crates.io/crates/takeout-metadata
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/aimericsr/takeout-metadata/blob/main/LICENSE

# CLI app for re-ordering Google Takeout export

When we do an export via Google Takeout, photos, videos and screenshort are exported with the wrong created date. But each file have a metadata file (.json) that containes this inforamtions. This CLI app traverse a directory to update the metadata of each file with the good one. This support photos(jpg) and videos(mp4).

### Install with cargo
```
cargo install takeout-metadata
```
