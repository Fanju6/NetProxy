# NetProxy 开发指南

欢迎加入 NetProxy 项目开发！本文档旨在帮助新成员快速了解项目结构、搭建开发环境并参与功能开发。

## 1. 项目简介

NetProxy 是一款基于 [Tauri](https://tauri.app/) 构建的轻量级 Windows 代理客户端。
- **核心引擎**: 集成 [Xray-core](https://github.com/XTLS/Xray-core) 处理网络流量。
- **用户界面**: 使用 Vue 3 + MDUI 2 (Material Design) 提供现代化的操作体验。

## 2. 技术栈

| 领域 | 技术/框架 | 说明 |
|------|-----------|------|
| **前端** | Vue 3 (Composition API) | 响应式 UI 框架 |
| | TypeScript | 类型安全保证 |
| | Vite | 极速构建工具 |
| | MDUI 2 | Material Design Web Components 组件库 |
| **后端** | Rust | 高性能、内存安全的系统级语言 |
| | Tauri v2 | 跨平台应用框架，提供系统调用能力 |
| **核心** | Xray-core | 代理流量转发核心 |
| **工具** | ProxyLink | (自研/集成) 负责订阅解析和节点格式转换 |

## 3. 环境准备

在开始编写代码之前，请确保你的开发环境已安装以下工具：

1.  **Node.js**: 推荐 v25 或更高版本。
2.  **Package Manager**: `npm` 或 `pnpm` / `yarn`。
3.  **Rust**: 安装最新稳定版 Rust (使用 [Rustup](https://rustup.rs/))。
4.  **C++ Build Tools**: 安装 Visual Studio 生成工具 (勾选 "使用 C++ 的桌面开发")，用于编译 Rust 的 Windows 依赖。

## 4. 快速上手

### 4.1 克隆与安装

```bash
# 1. 克隆仓库
git clone https://github.com/your-repo/NetProxy.git
cd NetProxy

# 2. 安装前端依赖
npm install
```

### 4.2 资源文件准备

本项目依赖外部二进制文件运行。在开发前，请确保 `src-tauri/resources/NetProxy` 目录结构完整：

```
src-tauri/resources/NetProxy/
├── bin/
│   └── xray.exe          # Xray 核心程序
├── config/
│   └── xray/             # 配置文件目录
│       ├── confdir/      # 拆分配置 (01_inbounds.json 等)
│       └── outbounds/    # 节点配置文件存放处
├── tools/
│   └── proxylink/
│       └── proxylink.exe # 订阅解析工具
└── logs/                 # 运行日志
```


### 4.3 运行开发环境

```bash
# 启动 Tauri 开发窗口（包含前端热重载和后端编译）
npm run tauri dev
```

### 4.4 构建发布

```bash
# 构建 Windows 安装包 (.msi / .exe)
npm run tauri build
```
构建产物位于 `src-tauri/target/release/bundle/`。

## 5. 项目目录结构

```
NetProxy/
├── src/                    # [前端] Vue 源代码
│   ├── api/                # Tauri API 封装 (与后端通信的桥梁)
│   ├── assets/             # 静态资源 (图片, CSS)
│   ├── components/         # 公共组件
│   ├── types/              # TypeScript 类型定义
│   ├── views/              # 页面视图 (Dashboard, Nodes, Settings)
│   ├── App.vue             # 根组件 (含导航栏)
│   └── main.ts             # 入口文件
├── src-tauri/              # [后端] Rust 源代码
│   ├── src/
│   │   ├── commands/       # Tauri 命令 (暴露给前端的接口)
│   │   │   ├── config.rs   # 配置读写
│   │   │   ├── nodes.rs    # 节点管理 (导入/删除/列表)
│   │   │   └── proxy.rs    # 代理控制 (启动/停止/状态)
│   │   ├── models/         # Rust 结构体定义 (对应前端 types)
│   │   ├── services/       # 核心业务服务
│   │   │   ├── xray.rs     # Xray 进程管理
│   │   │   ├── system_proxy.rs # Windows 系统代理设置
│   │   │   └── proxylink.rs    # 订阅解析服务
│   │   ├── utils/          # 工具函数 (路径处理等)
│   │   └── lib.rs          # 库入口，注册命令
│   ├── resources/          # 运行时附加资源
│   ├── icons/              # 应用图标
│   ├── tauri.conf.json     # Tauri 项目配置
│   └── Cargo.toml          # Rust 依赖配置
├── package.json            # Node.js 依赖配置
└── vite.config.ts          # Vite 配置
```

## 6. 核心功能开发指南

### 添加新的 Tauri 命令
1. 在 `src-tauri/src/commands/` 下创建或修改对应的 `.rs` 文件。
2. 定义 `#[tauri::command]` 装饰的函数。
3. 在 `src-tauri/src/lib.rs` 的 `invoke_handler` 中注册该命令。
4. 在 `src/api/` 中添加对应的 TypeScript 封装函数。

### 修改 Xray 配置逻辑
- Xray 的启动参数和进程管理位于 `src-tauri/src/services/xray.rs`。
- 配置文件模板位于 `src-tauri/resources/NetProxy/config/xray/confdir/`。
- 若需修改入站/出站逻辑，请编辑对应的 json 模板文件。

### 调试
- **前端调试**: 使用 Edge/Chrome 开发者工具 (F12) 查看 Console 和 Network。
- **后端调试**: `println!` 宏输出的内容会显示在 VSCode 终端或运行窗口的控制台中。
- **Xray 日志**: 在应用的 "设置" -> "运行日志" 中查看，或直接查看 `logs/` 目录下的文件。