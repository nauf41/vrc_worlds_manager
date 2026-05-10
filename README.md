# vrc_worlds_manager
## About
This program lets it easier and safer to manage VRChat's worlds bookmark.

## Features
* Register unlimited bookmarks
* Use unlimited and multiple applicable tags
* Core features completely works offline
* Simple Discord integrations
  * Load from Discord channel
  * Post to Discord channel
  * Link tags and Discord channels
* No credentials required
* Completely complies with ToS

## Technical Stack
* chrome extension: TypeScript + Webpack
* bridge: Rust
* world_manager: Tauri (Rust)

## To compile and install
1. prepare a sqlite DB (refer to `world_manager/src-tauri/src/db.rs` for structure; rows are not needed)
2. compile bridge: `cd bridge` then `cargo build --release`
3. compile chrome extensions: `cd chrome_extension`, `npm i` then `npm run build`
4. compile world manager: `cd world_manager`, `pnpm i` then `pnpm tauri build` (it generates installer, but it's not used now)
5. use the powershell script in latest release and install native messaging
