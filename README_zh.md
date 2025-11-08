# midmap
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jingangdidi/mindmap/blob/master/LICENSE)

[English readme](https://github.com/jingangdidi/mindmap/blob/master/README.md)

**A lightweight mindmap command-line tool (~1.7Mb)**

**轻量级命令行思维导图工具，无需安装，仅一个可执行文件（~1.7Mb）**

## 👑 特点
- 💪​ 单个可执行文件（~1.7Mb），无需安装
- 🎨​ 基于[mind-elixir](https://github.com/SSShooter/mind-elixir-core)
- 1️⃣​ 支持保存为单个HTML文件
- 🌄 支持保存为PNG图片

## 🚀 使用说明
**1. 下载预编译的可执行文件**

  [latest release](https://github.com/jingangdidi/mindmap/releases)

**2. 开启服务并在浏览器使用**

```
./mindmap
```
在浏览器访问[http:127.0.0.1:8081](http:127.0.0.1:8081)

## 🕹 页面右上角功能说明
<img src="https://github.com/jingangdidi/mindmap/raw/master/assets/top-right.png">

- 下拉选取之前的思维导图，在思维导图之间快速切换
- 给当前思维导图一个简短描述（可选）
- <img src="https://github.com/jingangdidi/mindmap/raw/master/assets/plus-add-create-new-cross-svgrepo-com.svg" width="18" height="18" align="center"> 创建新思维导图
- <img src="https://github.com/jingangdidi/mindmap/raw/master/assets/download-square-svgrepo-com.svg" width="18" height="18" align="center"> 将当前思维导图保存为单个html文件
- <img src="https://github.com/jingangdidi/mindmap/raw/master/assets/image-svgrepo-com.svg" width="18" height="18" align="center"> 将当前思维导图保存为png图片
- <img src="https://github.com/jingangdidi/mindmap/raw/master/assets/cloud-upload-svgrepo-com.svg" width="18" height="18" align="center"> 记录当前思维导图的状态，这个很重要，如果编辑完的思维导图没有被记录，关闭页面后将丢失

## 🛠 从源码编译
```
git clone https://github.com/jingangdidi/mindmap.git
cd mindmap
cargo build --release
```

## 🚥 命令行参数
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

## 📚 参数文件（可选）
```
Config(
    addr:     "127.0.0.1",
    port:     8081,
    language: "en",
    outpath:  "./mindmap",
)
```
- 参数优先级：命令行参数 > 参数文件 > 默认值
- 程序会自动在当前路径和程序所在路径下寻找[mindmap_config.txt](https://github.com/jingangdidi/mindmap/raw/master/mindmap_config.txt)，没有指定命令行参数，且参数文件不存在，则会使用默认值

## ❤️ 参考
[mind-elixir](https://github.com/SSShooter/mind-elixir-core)

## ⏰ 更新记录
- [2025.11.08] release [v0.1.2](https://github.com/jingangdidi/mindmap/releases/tag/v0.1.2)
  - 🛠修复：将`'`转义为`\'`，`\`转义为`\\`
  - 💪🏻优化: 隐藏滚动条：`overflow: hidden;`
- [2025.11.06] release [v0.1.1](https://github.com/jingangdidi/mindmap/releases/tag/v0.1.1)
  - 🛠修复：将`-->`转义为`--\\>`
- [2025.10.31] release v0.1.0
