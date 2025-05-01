# a-video

## 项目简介

a-video 是一个基于 Tauri、Vue 3 和 TypeScript 构建的桌面视频应用，结合 Rust 后端，提供高性能和跨平台的用户体验。项目旨在为用户提供便捷的视频采集、播放与管理功能。

## 主要功能
- 视频采集与管理
- 视频播放
- 数据持久化存储
- 多平台支持（Windows、macOS、Linux）
- 现代化 UI 界面

## 技术栈
- [Tauri](https://tauri.app/)：桌面端应用开发框架，结合 Web 前端与 Rust 后端
- [Vue 3](https://vuejs.org/)：渐进式前端框架
- [TypeScript](https://www.typescriptlang.org/)：类型安全的 JavaScript 超集
- [Rust](https://www.rust-lang.org/)：高性能后端语言
- 其他依赖：tokio、reqwest、rusqlite、serde、tauri-plugin-opener、tauri-plugin-shell、tauri-plugin-dialog 等

## 安装与运行

### 环境准备
1. 安装 [Node.js](https://nodejs.org/)（建议 v16 及以上）
2. 安装 [pnpm](https://pnpm.io/)（或 npm/yarn）
3. 安装 [Rust](https://www.rust-lang.org/tools/install)
4. 安装 [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### 安装依赖
```bash
pnpm install
```

### 启动开发环境
```bash
pnpm tauri dev
```

### 构建发布版本
```bash
pnpm tauri build
```

## 目录结构说明
```
├── src-tauri/           # Rust 后端与 Tauri 配置
│   ├── src/             # Rust 源码目录
│   ├── icons/           # 应用图标资源
│   ├── resources/       # 其他资源文件
│   ├── Cargo.toml       # Rust 依赖配置
│   └── tauri.conf.json  # Tauri 配置文件
├── src/                 # 前端 Vue 3 + TypeScript 源码
│   ├── components/      # Vue 组件
│   ├── pages/           # 页面组件
│   ├── stores/          # 状态管理
│   ├── router/          # 路由配置
│   ├── types/           # 类型定义
│   └── assets/          # 静态资源
├── public/              # 公共静态资源
├── package.json         # 前端依赖配置
├── pnpm-lock.yaml       # pnpm 锁定文件
├── vite.config.ts       # Vite 配置
└── README.md            # 项目说明文档
```

## 常见问题
1. **依赖安装失败**：请确保 Node.js、pnpm、Rust 均已正确安装，并使用国内镜像源（如有需要）。
2. **Tauri 构建报错**：请检查 Rust 工具链和 Tauri CLI 是否安装，或尝试升级到最新版本。
3. **界面样式异常**：请确认 tailwindcss 等依赖已正确安装。

## 联系方式
如有问题或建议，欢迎通过 Issue 反馈或联系开发者。

---

> 推荐开发环境：VS Code + Volar + Tauri 插件 + rust-analyzer

更多开发建议与详细说明请参考源码注释及相关文档。
