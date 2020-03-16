# Telegram Bot REST Service

REST service for Telegram Bot.

## Getting started

Copy example file `telegram-bot.conf-dist` to `telegram-bot.conf`.

Edit config file.

Run app.

## Proxy support

Add environment variables:

```
HTTP_PROXY=http://yourhost:3128
HTTPS_PROXY=http://yourhost:3128
```

Run app.

## How to build app from source

### CentOS 7

Install environment dependencies:

```shell script
yum group install 'Development Tools'
yum install openssl-devel
```

Build:

```shell script
cargo build --release
```

### Windows

Remove dependency `openssl` from `Cargo.toml`.

Then build:

Build:

```shell script
cargo build --release
```