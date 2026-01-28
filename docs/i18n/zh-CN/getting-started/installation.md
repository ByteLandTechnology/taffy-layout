---
title: 安装
sidebar_position: 1
---

# 安装

```bash
npm install taffy-layout
```

## 环境要求

- **Node.js**: 版本 12 或更高。
- **浏览器**: 支持 **WebAssembly** 的现代浏览器：
  - Chrome 57+
  - Firefox 52+
  - Safari 11+
  - Edge 16+

## 浏览器使用

通过 ES Modules 在浏览器中使用 Taffy：

```html
<script type="module">
  import {
    loadTaffy,
    TaffyTree,
  } from "https://cdn.jsdelivr.net/npm/taffy-layout/dist/index.js";

  // 1. 初始化 WebAssembly（必需）
  await loadTaffy();

  // 2. 开始使用 Taffy
  const tree = new TaffyTree();
</script>
```

## Node.js 使用

在 Node.js 环境中使用 Taffy：

```typescript
import { loadTaffy, TaffyTree } from "taffy-layout";

// 1. 初始化 WebAssembly（必需）
await loadTaffy();

// 2. 开始使用 Taffy
const tree = new TaffyTree();
```

## TypeScript 支持

Taffy Layout 开箱即用，包含**完整的 TypeScript 类型定义**。无需额外安装（如 `@types/taffy-layout`）。您将获得 `Style`、`Layout` 和 `TaffyTree` 的完整智能提示。

## 下一步

- **[快速开始指南](./quick-start.md)** - 创建您的第一个 Taffy 布局。
- **[API 参考](../../api/index.md)** - 查看完整的 API 文档。
