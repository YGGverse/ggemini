# ggemini

Glib-oriented client for [Gemini protocol](https://geminiprotocol.net/)

> [!IMPORTANT]
> Project in development!
>

## Install

```
cargo add ggemini
```

## Usage

## `client`


#### `client::simple_socket_request_async`

High-level API to make async socket request and auto-close connection on complete.

Return [Response](#clientresponseresponse) on success or [Error](#clienterror) enum on failure.

``` rust
use glib::{Uri, UriFlags};

match Uri::parse("gemini://geminiprotocol.net/", UriFlags::NONE) {
    // Begin async request
    Ok(uri) => ggemini::client::simple_socket_request_async(uri, |response| match response {
        // Process response
        Ok(response) => {
            // Expect success status
            assert!(match response.header().status() {
                Some(ggemini::client::response::header::Status::Success) => true,
                _ => false,
            })
        }
        Err(_) => assert!(false),
    }),
    Err(_) => assert!(false),
}
```

#### `client::Error`

#### `client::response`
#### `client::response::Response`

#### `client::response::header`
#### `client::response::header::meta`
#### `client::response::header::mime`
#### `client::response::header::status`
#### `client::response::header::language`
#### `client::response::header::charset`

#### `client::response::body`

#### `client::socket`
#### `client::socket::connection`
#### `client::socket::connection::input`
#### `client::socket::connection::input::buffer`
#### `client::socket::connection::output`

## Integrations

* [Yoda](https://github.com/YGGverse/Yoda) - Browser for Gemini Protocol

## See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented Gemtext API