# shrtnr
Url shortener written in [Rust](https://www.rust-lang.org/) *just to touch a totally new stack*.

## Used libs
* [Rocket](https://rocket.rs/) - framework
* [Sled](https://docs.rs/sled/0.34.7/sled/) - storage layer
* [Harsh](https://docs.rs/harsh/0.2.2) - ids hashing

## Configuration
Configuration values are loaded from environment, thanks to [config-rs](https://docs.rs/config/0.13.3).
| Name | Required | Description |
| ---- | --- | ----------- |
| SHRTNT__HOST | ✅ | Base url for generated URL |
| HARSH__SALT | | Init value for Harsh |
| HARSH__LENGTH | | Minimal hash length |
| HARSH__ALPHABET | | Symbols allowed in hash |
