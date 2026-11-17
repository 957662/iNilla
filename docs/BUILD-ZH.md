# iNilla 中文化改造说明（本地补丁，未构建）

本目录是对 https://github.com/Martze96/iNilla 的本地改造，用于让客户端能显示中文。

## 已做的改动
1. crates/benilla-app/src/ui_text/engine/faces.rs
   - CLIENT_FONTS 增加 Fonts\\NotoSansSC.TTF（放在 Friz 之后作为 CJK 回退面）。
   - 配套：服务器 Data 的 fonts.MPQ 已加入该字体，并把 FRIZQT__.TTF / ARIALN.TTF 替换为 Noto Sans SC 子集（拉丁+CJK，约 9.5MB）。

## 还没做（需要构建环境）
- 编码层：若以后使用 zhCN 客户端数据（GBK），需要在 chain.rs 的 read 与 benilla-ui/src/source.rs 加按 locale 选择码页的解码（当前 enUS/UTF-8 不受影响）。
- egui 自有界面（设置页/HUD）字体注入：crates/inilla-app/src/inilla/ui/mod.rs 附近。

## 构建方式（二选一）
A. GitHub Actions（推荐）
 1. Fork https://github.com/Martze96/iNilla 到自己账号
 2. 推上本目录改动
 3. Actions -> inilla-ipa -> Run workflow
 4. 下载 artifact 里的 iNilla-*.ipa，用 SideStore/AltStore 安装

B. 本机 macOS 构建
 需要 Xcode（约 15GB）、Rust（rustup）、rustup target add aarch64-apple-ios
 cargo build --release --target aarch64-apple-ios -p inilla-ios
 bash ios/package-ipa.sh
 （详见 docs/IOS-BUILD.md）

## 当前状态
- 服务器端：中文数据库补丁已应用（9.7 万条 NPC/任务/物品文本），Data 的 fonts.MPQ 已带中文字体。
- 客户端：未重编，仅替换字体 MPQ；界面按钮/菜单仍为英文（来自 interface.MPQ）。
