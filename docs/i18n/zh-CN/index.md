---
title: Taffy Layout - 高性能布局引擎
hero:
  name: Taffy Layout
  text: 高性能布局引擎
  tagline: 为 Web、移动端及更多场景打造的灵活、高性能 Flexbox 和 Grid 布局引擎。基于 Rust 和 WASM 构建。
  image:
    src: /images/hero_illustration.svg
    alt: Taffy Layout 动画插图
  actions:
    - theme: brand
      text: 文档首页
      link: /zh/docs
    - theme: alt
      text: API 参考
      link: /zh/docs/api
    - theme: alt
      text: 基准测试
      link: /benchmark
    - theme: alt
      text: Playground
      link: /playground
features:
  - title: 极速性能
    details: 使用 Rust 编写并编译为 WebAssembly，可通过基准测试页面了解布局计算性能。
  - title: Flexbox 与 Grid
    details: 支持 Flexbox、CSS Grid 和块布局，返回可用于自定义渲染的尺寸和位置。
  - title: 现代标准
    details: 采用熟悉的 CSS 属性名称；可用属性与取值见 API 参考。
  - title: 多平台支持
    details: 可集成到支持所需 WebAssembly 特性的 JavaScript 环境，包括 Web 应用和自定义渲染器。
---

# Taffy Layout

欢迎来到 Taffy Layout 官方文档。

Taffy 是一款支持 Flexbox 和 CSS Grid 的布局引擎。它旨在提供高性能、轻量级且易于在不同平台之间使用的布局解决方案。

## 核心目标

- **正确性**：严格遵守 CSS 规范。
- **性能**：针对速度和低内存占用进行优化。
- **移植性**：通过 WebAssembly 集成到满足[运行环境要求](./getting-started/installation.md)的 JavaScript 应用。
