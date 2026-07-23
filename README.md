# midmap
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jingangdidi/mindmap/blob/master/LICENSE)

[中文文档](https://github.com/jingangdidi/mindmap/blob/master/README_zh.md)

**A lightweight mindmap command-line tool (~2Mb)**

<img src="https://github.com/jingangdidi/mindmap/raw/master/assets/example.png">

## 👑 Features
- 💪​ Single-file executable (~2Mb) - no installation required
- 🎨​ Based on [mind-elixir](https://github.com/SSShooter/mind-elixir-core)
- 1️⃣​ Support save as single HTML file
- 🌄 Support save as PNG image
- ✨ Support node highlighting

## 🚀 Usage
**1. download a pre-built binary**

  [latest release](https://github.com/jingangdidi/mindmap/releases)

**2. start server & visit via your browser**

```
./mindmap
```
visit via your browser [http:127.0.0.1:8081](http:127.0.0.1:8081)

## 🕹 Top right button
<img src="https://github.com/jingangdidi/mindmap/raw/master/assets/top-right.png">

- Pull down to select the previous mindmap and quickly switch between mindmaps
- Provide a brief description of the current mindmap (optional)
- Drop down to select node for highlighting
- Switch between read-only or editable
- <img src="https://github.com/jingangdidi/mindmap/raw/master/assets/plus-add-create-new-cross-svgrepo-com.svg" width="18" height="18" align="center"> Create a new mindmap
- <img src="https://github.com/jingangdidi/mindmap/raw/master/assets/download-square-svgrepo-com.svg" width="18" height="18" align="center"> Save the current mindmap as a single HTML file
- <img src="https://github.com/jingangdidi/mindmap/raw/master/assets/image-svgrepo-com.svg" width="18" height="18" align="center"> Save the current mindmap as a PNG image
- <img src="https://github.com/jingangdidi/mindmap/raw/master/assets/cloud-upload-svgrepo-com.svg" width="18" height="18" align="center"> Record the current state of the mindmap. If the edited mindmap is not recorded, it will be lost after closing the page

## 🛠 Building from source
```
git clone https://github.com/jingangdidi/mindmap.git
cd mindmap
cargo build --release
```

## 🚥 Arguments
```
Usage: mindmap [-a <addr>] [-p <port>] [-l <language>] [-o <outpath>] [-c <config>]

mindmap server, based on mind-elixir v5.1.1

Options:
  -a, --addr        ip address, default: 127.0.0.1
  -p, --port        port, default: 8081
  -l, --language    language, support: zh_CN, zh_TW, en, ja, pt, ru, default: en
  -o, --outpath     output path, default: ./mindmap
  -c, --config      config file, the priority of -a/-p/-l/-o is higher than -c, default: mindmap_config.txt in current path or binary executable file path
  -h, --help        display usage information
```

## 📚 config file (optional)
```
Config(
    addr:     "127.0.0.1",
    port:     8081,
    language: "en",
    outpath:  "./mindmap",
)
```
- config priority: command line parameters > config file > default
- The program will automatically search for [mindmap_config.txt](https://github.com/jingangdidi/mindmap/raw/master/mindmap_config.txt) in the current path and the path where the program is located. If no command-line parameters are specified and the config file does not exist, the default value will be used.

## ❤️ Acknowledgements
[mind-elixir](https://github.com/SSShooter/mind-elixir-core)

## ⏰ changelog
- [2026.07.23] release [v0.1.4](https://github.com/jingangdidi/mindmap/releases/tag/v0.1.4)
  - ⭐️Add: Add a save current mind map to local button in the upper right corner.
- [2025.11.12] release [v0.1.3](https://github.com/jingangdidi/mindmap/releases/tag/v0.1.3)
  - ⭐️Add: Switch button between read-only and editable
  - ⭐️Add: Select a node from the drop-down menu and highlight the background color of the node and its parent nodes, the common parent nodes will be set to gray.
- [2025.11.09] release [v0.1.2](https://github.com/jingangdidi/mindmap/releases/tag/v0.1.2)
  - 🛠Fix: Escape `'` as `\'`, `\` as `\\`
  - 🛠Fix: The label is removed when the record button is clicked next time.
  - 🛠Fix: If the label's txt file was previously saved locally, and the label is removed from the page this time, the local label file needs to be deleted.
  - 💪🏻Optimize: hide scroll bar: `overflow: hidden;`
- [2025.11.06] release [v0.1.1](https://github.com/jingangdidi/mindmap/releases/tag/v0.1.1)
  - 🛠Fix: Escape `-->` as `--\\>`
- [2025.10.31] release [v0.1.0](https://github.com/jingangdidi/mindmap/releases/tag/v0.1.0)
