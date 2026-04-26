# Bridge
Chrome拡張機能とTauriアプリをつなげるブリッジで、複数プロセスに対応しています。

Bridge-Tauri
| Bit | Val |
| - | - |
| 0-31 | クライアントID (little-endian) |
| 32-63 | メッセージサイズ (little-endian) |
| 64- | メッセージ本体 |

Bridge-Chrome
| Bit | Val |
| - | - |
| 0-31 | メッセージサイズ (little-endian) |
| 64- | メッセージ本体 |
