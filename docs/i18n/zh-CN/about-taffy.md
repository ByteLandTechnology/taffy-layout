---
title: 关于 Taffy
sidebar_position: 1
---

# 关于 Taffy

Taffy 是一个高性能、可嵌入的布局引擎，使用 Rust 编写并编译为 WebAssembly。它为 JavaScript 带来了接近原生性能的 CSS Flexbox 和 Grid 布局算法。

## Taffy 的独特之处

### WebAssembly 驱动

Taffy 使用 Rust 编写并编译为 WebAssembly，其性能特征比纯 JavaScript 实现更接近原生代码。

### 完整的 CSS 支持

Taffy 实现了 CSS Flexbox 和 CSS Grid 布局引擎，与 Web 浏览器具有完整的功能对等性。

### 基于树的 API

Taffy 提供高效的树结构来管理布局节点，非常适合复杂的嵌套布局。

### 自定义测量

Taffy 支持动态内容（如文本）的自定义测量回调，支持终端 UI、Canvas 渲染等场景。

## 使用场景

### 终端 UI

非常适合需要精确布局计算的终端 UI 框架。

### Canvas 应用

适合需要高效布局计算的基于 Canvas 的渲染引擎。

### 自定义渲染引擎

非常适合需要类 CSS 布局但没有 DOM 的自定义 UI 框架。

## 架构

Taffy 基于 [Taffy Rust 库](https://github.com/DioxusLabs/taffy)，具有专门为 JavaScript 和 TypeScript 设计的 WebAssembly 绑定。

### 核心模块

- **Style**: CSS 布局属性配置
- **TaffyTree**: 布局树管理和计算
- **Layout**: 计算后的布局结果（位置、尺寸等）
- **Enums**: CSS 属性枚举（Display、FlexDirection 等）
- **Types**: TypeScript 类型定义

## 为什么不直接使用 CSS？

虽然 CSS 在 Web 浏览器中表现出色，但 Taffy 填补了重要的空白：

1. **终端 UI**: CSS 在终端环境中不起作用
2. **Canvas 渲染**: 没有 DOM 意味着没有 CSS 支持
3. **自定义引擎**: 使用类 CSS 布局构建您自己的 UI 框架
4. **性能**: 高效的 WebAssembly 实现适用于高性能应用
