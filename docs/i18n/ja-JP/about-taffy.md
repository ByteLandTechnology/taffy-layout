---
title: Taffy について
sidebar_position: 1
---

# Taffy について

Taffy は、Rust で記述され WebAssembly にコンパイルされた高性能で埋め込み可能なレイアウトエンジンです。ネイティブに近いパフォーマンスで CSS Flexbox と Grid レイアウトアルゴリズムを JavaScript に提供します。

## Taffy の特徴

### WebAssembly パワード

Taffy は Rust で記述され WebAssembly にコンパイルされているため、純粋な JavaScript 実装よりもネイティブコードに近いパフォーマンス特性を持っています。

### 完全な CSS サポート

Taffy は CSS Flexbox と CSS Grid レイアウトエンジンの両方を、Web ブラウザと完全な機能パリティで実装しています。

### ツリーベース API

Taffy はレイアウトノードを管理するための効率的なツリー構造を提供し、複雑なネストされたレイアウトに最適です。

### カスタム測定

Taffy はテキストなどの動的コンテンツ用のカスタム測定コールバックをサポートし、ターミナル UI、Canvas レンダリングなどを可能にします。

## ユースケース

### ターミナル UI

正確なレイアウト計算が重要なターミナルベースの UI フレームワークに最適です。

### Canvas アプリケーション

効率的なレイアウト計算が必要な Canvas ベースのレンダリングエンジンに理想的です。

### カスタムレンダリングエンジン

DOM なしで CSS ライクなレイアウトが必要なカスタム UI フレームワークに最適な選択です。

## アーキテクチャ

Taffy は [Taffy Rust ライブラリ](https://github.com/DioxusLabs/taffy) をベースとし、JavaScript と TypeScript 用に特別に設計された WebAssembly バインディングを備えています。

### コアモジュール

- **Style**: CSS レイアウトプロパティ設定
- **TaffyTree**: レイアウトツリー管理と計算
- **Layout**: 計算されたレイアウト結果（位置、サイズなど）
- **Enums**: CSS プロパティ列挙型（Display、FlexDirection など）
- **Types**: TypeScript 型定義

## なぜ CSS を使わないのか？

CSS は Web ブラウザには最適ですが、Taffy は重要なギャップを埋めます：

1. **ターミナル UI**: CSS はターミナル環境では機能しません
2. **Canvas レンダリング**: DOM がないということは CSS サポートがないということです
3. **カスタムエンジン**: CSS ライクなレイアウトで独自の UI フレームワークを構築
4. **パフォーマンス**: 高効率な WebAssembly 実装は高性能アプリケーションに適しています
