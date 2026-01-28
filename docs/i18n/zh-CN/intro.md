---
sidebar_position: 1
---

# Taffy Layout 文档

**高性能、跨平台的 UI 布局引擎。**

Taffy Layout 是一个用 Rust 编写并可编译为 WebAssembly (JS/TS) 的高性能布局引擎。它实现了浏览器级的 **Flexbox** 和 **Grid** 算法，且不依赖 DOM，非常适合终端 UI、Canvas 渲染器、游戏 UI 和自定义渲染引擎。

## 你可以用它构建什么

- **无 DOM 布局**：为 Canvas、WebGL 或终端应用提供 CSS 风格的布局。
- **海量节点树**：处理大型 UI 树，具有快速、确定性的性能。
- **自定义测量**：支持文本、图像和动态内容的尺寸测量。
- **可复用模式**：清晰映射到 UI 组件的布局模式。

## 文档地图

| 章节                                       | 描述                              |
| :----------------------------------------- | :-------------------------------- |
| **[入门指南](./getting-started/index.md)** | 安装、初始化和你的第一个布局。    |
| **[核心概念](./core-concepts/index.md)**   | 尺寸模型、可用空间和测量。        |
| **[样式](./styling/index.md)**             | Flexbox、Grid、间距、尺寸和定位。 |
| **[实例手册](./cookbook/index.md)**        | 常见 UI 的即用型布局模式。        |
| **[高级主题](./advanced/index.md)**        | 调试、性能优化和错误处理。        |
| **[API 参考](../api/index.md)**            | 详细的 API 文档（自动生成）。     |

## 极简心智模型

使用 Taffy 只需要三个简单的步骤：

1.  **构造 (Construct)**：构建带有样式的节点树。
2.  **计算 (Compute)**：根据可用空间计算布局。
3.  **读取 (Read)**：获取计算出的 `x`、`y`、`width` 和 `height`。

```mermaid
graph LR
    A[定义样式] --> B[构建树]
    B --> C[计算布局]
    C --> D[读取结果]
```

```text
UI 树结构:
└── TaffyTree
    ├── Style (输入: Flex, Width, Padding 等规则)
    └── Layout (输出: 计算后的 X, Y, Width, Height)
```

## 下一步

- **[快速开始](./getting-started/quick-start.md)** - 在几分钟内构建你的第一个布局。
- **[样式指南](./styling/index.md)** - 学习如何控制布局。
- **[布局实例手册](./cookbook/index.md)** - 复制粘贴真实世界的示例。
