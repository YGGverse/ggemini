# ggemini

Glib/Gio-oriented network API for [Gemini protocol](https://geminiprotocol.net/)

> [!IMPORTANT]
> Project in development!
>

GGemini (or G-Gemini) library written as the client extension for [Yoda](https://github.com/YGGverse/Yoda) - GTK Browser for Gemini Protocol,
it also could be useful for any other integrations as depend of [glib](https://crates.io/crates/glib) and [gio](https://crates.io/crates/gio) (`2.66+`) crates only

## Install

```
cargo add ggemini
```

## Usage

_todo_

### `client`

[Gio](https://docs.gtk.org/gio/) API already provides powerful [SocketClient](https://docs.gtk.org/gio/class.SocketClient.html)\
`client` collection just extends some features wanted for Gemini Protocol interaction.

#### `client::response`
#### `client::response::header`
#### `client::response::body`

### `gio`

#### `gio::memory_input_stream`

## See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented Gemtext API