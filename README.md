# A-VIDEO 跨平台视频应用

![Project Banner](public/A-VIDEO.png)

[![Vue3](https://img.shields.io/badge/Vue-3.5%2B-brightgreen)](https://vuejs.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.5-blue)](https://tauri.app/)
[![SQLite](https://img.shields.io/badge/SQLite-3-green)](https://sqlite.org)

```mermaid
graph TD
    A[核心功能] --> B[视频播放]
    A --> C[收藏管理]
    A --> D[跨平台支持]
    B --> B1[HLS流媒体]
    B --> B2[播放列表]
    C --> C1[本地存储]
    C --> C2[智能分类]
    D --> D1[Windows]
    D --> D2[macOS]
    D --> D3[Linux]
```

## 技术栈架构

```mermaid
pie
    title 技术构成
    "Vue3前端" : 45
    "Tauri后端" : 30
    "Rust核心" : 15
    "SQLite存储" : 10
```

## 快速开始

### 环境要求
- Node.js 18+
- Rust 1.70+
- pnpm 8+

### 安装步骤
```bash
# 前端依赖
pnpm install

# Tauri构建环境
pnpm tauri init
```

### 开发模式
```bash
# 启动开发服务器
pnpm dev

# 运行桌面应用
pnpm tauri dev
```

## 项目配置
```mermaid
flowchart LR
    config[.env] --> vite[Vite配置]
    config --> tauri[Tauri配置]
    config --> db[数据库配置]
    
    subgraph 环境变量
    VITE_API_ENDPOINT
    VITE_TAURI_DEBUG
    DATABASE_PATH
    end
```

## 贡献指南

1. Fork项目仓库
2. 创建特性分支 (`git checkout -b feature/新功能`)
3. 提交更改 (`git commit -m '添加新功能'`)
4. 推送分支 (`git push origin feature/新功能`)
5. 创建Pull Request

## 许可证
MIT License - 详见 [LICENSE](LICENSE)

---

> 项目文档持续更新中，最新技术细节请参考 [API文档](src/API.txt)
