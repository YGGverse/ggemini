# ggemini

Glib/Gio-oriented network API for [Gemini protocol](https://geminiprotocol.net/)

> [!IMPORTANT]
> Project in development!
>

This library written as the network extension for [Yoda](https://github.com/YGGverse/Yoda) - GTK Browser for Gemini Protocol,
it also could be useful for any other integrations as depend of [glib](https://crates.io/crates/glib) and [gio](https://crates.io/crates/gio) (`2.66+`) crates only.

## Install

```
cargo add ggemini
```

## Usage

### `client`

[Gio](https://docs.gtk.org/gio/) API already provide powerful [SocketClient](https://docs.gtk.org/gio/class.SocketClient.html)\
`client` collection just extends some minimal features wanted for Gemini Protocol.

#### `client::response`
#### `client::response::header`
#### `client::response::body`

## See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented Gemtext API