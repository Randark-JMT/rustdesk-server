# RustDesk Server Program

[![build](https://github.com/rustdesk/rustdesk-server/actions/workflows/build.yaml/badge.svg)](https://github.com/rustdesk/rustdesk-server/actions/workflows/build.yaml)

[**Download**](https://github.com/rustdesk/rustdesk-server/releases)

[**Manual**](https://rustdesk.com/docs/en/self-host/)

[**Configuration & environment variables**](docs/environment-variables.md)

[**FAQ**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

[**How to migrate OSS to Pro**](https://rustdesk.com/docs/en/self-host/rustdesk-server-pro/installscript/#convert-from-open-source)

Self-host your own RustDesk server, it is free and open source.

> [!IMPORTANT]
> **Need more features?** [RustDesk Server Pro](https://rustdesk.com/pricing.html) might suit you better.
>
> **Want to develop your own server?** Start with [rustdesk-server-demo](https://github.com/rustdesk/rustdesk-server-demo), a simpler starting point than this repository.

## 此分支的增强特性（移植自 [lejianwen/rustdesk-server](https://github.com/lejianwen/rustdesk-server)）

- **强制登录**：`MUST_LOGIN=Y` 或 `hbbs --must-login=Y` 后，客户端必须携带登录 token 才能建立连接；默认为 `N`。
- **JWT 校验**：设置 `RUSTDESK_API_JWT_KEY` 后，`hbbs` 会通过 JWT（HS256）校验客户端 token 的合法性，配合 [rustdesk-api](https://github.com/lejianwen/rustdesk-api) 使用。
- **hbbs TCP/WebSocket 连接加密**：当配置了服务端密钥（`-k`）时，hbbs 的 TCP 端口会与客户端执行两阶段 KeyExchange（每连接独立的 `box_` 密钥对 + secretbox 对称加密），保护信令通道。
- **WebSocket 注册/打洞支持**：纯 WebSocket 客户端（>= 1.4.1）可在 ws 端口（21118）完成 RegisterPeer/RegisterPk/OnlineRequest 与打洞，并正确刷新在线状态。
- **S6 镜像集成 rustdesk-api**：`docker/Dockerfile` 基于 `lejianwen/rustdesk-api` 构建，镜像内包含 Web 管理端（端口 21114），可通过 `RUSTDESK_API_*` 环境变量配置。

## How to build manually

```bash
cargo build --release
```

Three executables will be generated in target/release.

- hbbs - RustDesk ID/Rendezvous server
- hbbr - RustDesk relay server
- rustdesk-utils - RustDesk CLI utilities

You can find updated binaries on the [Releases](https://github.com/rustdesk/rustdesk-server/releases) page.

## Configuration

`hbbs` and `hbbr` can be configured with command-line flags, environment
variables, or an `.env` / config file. Run `hbbs --help` or `hbbr --help` to see
the available flags.

The most common options:

| Option | Flag | Env var | Applies to | Purpose |
| --- | --- | --- | --- | --- |
| Key | `-k` | `KEY` | hbbs, hbbr | `hbbs` loads/generates one by default |
| Bind address | `-b` | `BIND` | hbbs, hbbr | Local IP address to listen on (default: all interfaces; requires 1.1.17+) |
| Port | `-p` | `PORT` | hbbs, hbbr | Listening port (hbbs `21116`, hbbr `21117`) |
| Relay servers | `-r` | `RELAY-SERVERS` | hbbs | Override when the relay uses a different address or a non-standard port |
| Force relay | — | `ALWAYS_USE_RELAY` | hbbs | `Y` disables direct connections |
| Must login | — | `MUST_LOGIN` | hbbs | `Y` requires the client to carry a login token (see `--must-login`) |
| JWT key | — | `RUSTDESK_API_JWT_KEY` | hbbs | When set, login tokens are validated as JWT (HS256) signed with this key |
| Log level | — | `RUST_LOG` | hbbs, hbbr | e.g. `debug` (default `info`) |

See **[docs/environment-variables.md](docs/environment-variables.md)** for the
full list of variables, the file/flag/env precedence rules, database and relay
bandwidth tuning, Docker image variables, and examples.

## Installation

Please follow this [doc](https://rustdesk.com/docs/en/self-host/rustdesk-server-oss/)
