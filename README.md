[releases]: https://github.com/mibmo/kount/releases
[issues]: https://github.com/mibmo/kount/issues
[discussions]: https://github.com/mibmo/kount/discussions

[matrix-badge]: https://img.shields.io/matrix/kount:mib.dev?label=%23kount%3Amib.dev&logo=matrix&server_fqdn=matrix.mib.dev
[matrix-url]: https://matrix.to/#/#kount:mib.dev
[crates-badge]: https://img.shields.io/crates/v/kount
[crates-url]: https://crates.io/kount
[license-badge]: https://img.shields.io/crates/l/kount?label=license
[license-url]: LICENSE

[wp environment variable]: https://en.wikipedia.org/wiki/Environment_variable
[wp path]: https://en.wikipedia.org/wiki/Path_(computing)
[wp json]: https://en.wikipedia.org/wiki/JSON

[rust]: https://rust-lang.org
[rust target]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
[rustup]: https://rustup.rs
[crates.io]: https://crates.io
[tracing directive]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/struct.EnvFilter.html#directives

[obs]: https://obsproject.com
[obs streamlabs]: https://streamlabs.com/streamlabs-obs-live-streaming-software
[xsplit]: https://xsplit.com/broadcaster

# Kount [![#kount:mib.dev][matrix-badge]][matrix-url] [![crates.io][crates-badge]][crates-url] [![license][license-badge]][license-url]
A global input counter with a built-in webserver for convenient use with streaming software.

## Installation
Installing is fairly simple, provided you have a working [Rust][rust] installation.
If you don't have Rust installed, using [Rustup][rustup] to install it is highly recommended.

### Compiling from source
To build for your default [target][rust target], run
```sh
cargo build --release
```

#### Installing through [crates.io][crates.io]
Installing is as simple as running
```sh
cargo install kount
```
Once run, __Kount__ will be installed to your Cargo binary directory (unix: `$HOME/.cargo/bin`, windows: `%USERPROFILE%\.cargo\bin`) which should make `kount` available in your [path][wp path].

### Binaries
If you don't have a working Rust installation and don't want to setup one,
pre-compiled biaries are available from the [releases][releases] tab.

## Usage
<span id="configuration"></span>
#### Configuration
There are 4 [environment variables][wp environment variable] the user can use to control and configure the program. \
If you're not sure how to change these; __don't worry.__ The defaults will likely work for you.
- `KOUNT_ADDR` (default: `0.0.0.0`) \
	IP address the webserver should bind to.
- `KOUNT_PORT` (default: `3000`) \
	Port the webserver should listen on.
- `KOUNT_DIR` (default: `fields`) \
	Directory live counter files should be placed in.
- `RUST_LOG` (default: `kount=info`) \
	[Tracing filter directive][tracing directive] for console logs.

Throughout the remainder of this README, `$WEB` is assumed to be and __should be replaced__ with `http://$KOUNT_ADDR:$KOUNT_PORT`.
By default, `$WEB` is `http://0.0.0.0:3000` and therefore the webserver runs at [0.0.0.0:3000](http://0.0.0.0:3000).

### Using with streaming software
Most streaming software has the ability to embed and live-reload text files.
There are files corresponding to all fields, updating live.
Reference available in the [file API section](#api-file).

#### OBS (and its forks, such as [StreamLabs OBS][obs streamlabs])
[OBS][obs]

#### XSplit Broadcaster
[XSplit][xsplit]

---

If the streaming software you use isn't listed here, consider opening an [issue][issues] or starting a topic in [discussions][discussions] about it.
It's very likely that __Kount__ can still work with the software.


### API
There's a very rudementary web [JSON][wp json] _"API"_ endpoint available: `$WEB/json` returns all the counters as a JSON object.

<span id="api-file"></span>
#### Fields
Live-updating files with the values of fields.
The directory in which these files are placed [can be configured](#configuration), but are here assumed to be the default of `fields`.
The counter directory is in the directory __Kount__ was run from.
- `fields/keyboard` \
Keyboard press count

<span id="api-fields"></span>
#### Fields
Endspoints available for directly accessing counters.
Prepend `$WEB` to the endpoints listed below.
- `/field/keyboard` \
Keyboard press count
