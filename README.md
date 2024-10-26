# ggemini

Glib/Gio-oriented network library for [Gemini protocol](https://geminiprotocol.net/)

> [!IMPORTANT]
> Project in development!
>

GGemini (or G-Gemini) initially created as client extension for [Yoda Browser](https://github.com/YGGverse/Yoda),
but also could be useful for any other integration as depends of
[glib](https://crates.io/crates/glib) and [gio](https://crates.io/crates/gio) (`v2_66`) crates only.

## Install

```
cargo add ggemini
```

## Usage

### `client`

Gio API already includes powerful [SocketClient](https://docs.gtk.org/gio/class.SocketClient.html),
Client implementation is minimal and just extends a bit some features for Gemini Protocol.

#### `client::buffer`

#### `client::response`

Response parser for [InputStream](https://docs.gtk.org/gio/class.InputStream.html)

#### `client::response::Response`
#### `client::response::header`
#### `client::response::body`

https://docs.gtk.org/glib/struct.Bytes.html

## See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented Gemtext API