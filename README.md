# ggemini

![Build](https://github.com/YGGverse/ggemini/actions/workflows/build.yml/badge.svg)
[![Documentation](https://docs.rs/ggemini/badge.svg)](https://docs.rs/ggemini)
[![crates.io](https://img.shields.io/crates/v/ggemini.svg)](https://crates.io/crates/ggemini)

Glib/Gio-oriented network API for [Gemini protocol](https://geminiprotocol.net/)

> [!IMPORTANT]
> Project in development!
>

GGemini (or G-Gemini) library written as the client extension for [Yoda](https://github.com/YGGverse/Yoda), it also could be useful for other GTK-based applications dependent of [glib](https://crates.io/crates/glib) and/or [gio](https://crates.io/crates/gio) (`2.66+`) bindings.

## Requirements

<details>
<summary>Debian</summary>
<pre>
sudo apt install libglib2.0-dev</pre>
</details>

<details>
<summary>Fedora</summary>
<pre>
sudo dnf install glib2-devel</pre>
</details>

## Install

```
cargo add ggemini
```

## Usage

* [Documentation](https://docs.rs/ggemini/latest/ggemini/)

### Example

``` rust
use gio::*;
use glib::*;

use ggemini::client::{
    connection::{
        Request, Response,
        request::Gemini,
        response::meta::{Mime, Status}
    },
    Client, Error,
};

fn main() -> ExitCode {
    Client::new().request_async(
        Request::gemini(
            Uri::parse("gemini://geminiprotocol.net/", UriFlags::NONE).unwrap(),
        ),
        Priority::DEFAULT,
        Cancellable::new(),
        None, // optional `GTlsCertificate`
        |result: Result<Response, Error>| match result {
            Ok(response) => {
                // route by status code
                match response.meta.status {
                    // code 20, handle `GIOStream` by content type
                    Status::Success => match response.meta.mime.unwrap().value.as_str() {
                        // gemtext, see ggemtext crate to parse
                        "text/gemini" => todo!(),
                        // other content types
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
            Err(_) => todo!(),
        },
    );
    ExitCode::SUCCESS
}
```

## Other crates

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented Gemtext API