# ggemini

Glib/Gio-oriented network API for [Gemini protocol](https://geminiprotocol.net/)

> [!IMPORTANT]
> Project in development!
>

GGemini (or G-Gemini) library written as the client extension for [Yoda](https://github.com/YGGverse/Yoda), it also could be useful for other GTK-based applications with [glib](https://crates.io/crates/glib) and [gio](https://crates.io/crates/gio) (`2.66+`) dependency.

## Install

```
cargo add ggemini
```

## Usage

* [Documentation](https://docs.rs/ggemini/latest/ggemini/)

### Example

``` rust
use gtk::gio::*;
use gtk::glib::*;

use ggemini::client::{
    connection::{
        response::meta::{Mime, Status},
        Response,
    },
    Client, Error,
};

fn main() -> ExitCode {
    Client::new().request_async(
        Uri::parse("gemini://geminiprotocol.net/", UriFlags::NONE).unwrap(),
        Priority::DEFAULT,
        Cancellable::new(),
        None, // optional `GTlsCertificate`
        |result: Result<Response, Error>| match result {
            Ok(response) => {
                match response.meta.status {
                    // route by status code
                    Status::Success => match response.meta.mime {
                        // handle `GIOStream` by content type
                        Some(Mime::TextGemini) => todo!(),
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
            Err(e) => todo!("{e}"),
        },
    );
    ExitCode::SUCCESS
}
```

## See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented Gemtext API