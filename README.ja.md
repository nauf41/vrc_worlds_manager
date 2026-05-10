# vrc_worlds_manager
## 概要
このプログラムは、VRChatのワールドのブックマーク管理をより簡単かつ安全に行うためのものです。

## 機能
* ブックマークの無制限登録
* 複数のタグの無制限適用
* コア機能の完全オフライン動作
* シンプルなDiscord連携
  * Discordチャンネルからの読み込み
  * Discordチャンネルへの投稿
  * タグとDiscordチャンネルの紐付け
* 認証情報（クレデンシャル）不要
* 利用規約（ToS）に完全準拠

## 技術スタック
* Chrome拡張機能: TypeScript + Webpack
* ブリッジ: Rust
* world_manager: Tauri (Rust)

## コンパイルとインストール方法
1. SQLiteデータベースを準備する（構造については `world_manager/src-tauri/src/db.rs` を参照。レコードデータは不要です）
2. ブリッジのコンパイル: `cd bridge` を実行後、`cargo build --release`
3. Chrome拡張機能のコンパイル: `cd chrome_extension` を実行し、`npm i` の後に `npm run build`
4. world_managerのコンパイル: `cd world_manager` を実行し、`pnpm i` の後に `pnpm tauri build` （インストーラーが生成されますが、現在は使用しません）
5. 最新リリースにあるPowerShellスクリプトを使用して、ネイティブメッセージングをインストールする
