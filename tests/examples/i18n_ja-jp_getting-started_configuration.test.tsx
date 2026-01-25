import React from "react";
import { test } from "vitest";
import init, {
  TaffyTree,
  Style,
  // Add all other exports that might be needed
  Display,
  FlexDirection,
  AlignItems,
  AlignContent,
  JustifyContent,
  Position,
  FlexWrap,
  BoxSizing,
  GridAutoFlow,
  Overflow,
  AlignSelf,
  TextAlign,
  Dimension,
  AvailableSpace,
  Size,
  GridPlacement,
  Rect,
  LengthPercentage,
  LengthPercentageAuto,
  DetailedLayoutInfo,
  DetailedGridInfo,
  DetailedGridTracksInfo,
  DetailedGridItemsInfo,
  TrackSizingFunction,
  Point,
  TaffyError,
  Layout,
  MeasureFunction,
} from "taffy-layout";

// Global init for the suite
await init();

// Mock TaffyTreePreview component
const TaffyTreePreview = (_props: any) => null;

test("i18n_ja-JP_getting-started_configuration example 1", async () => {
  // 1,000 ノードを収容できるように初期化
  const tree = TaffyTree.withCapacity(1000);
  console.log(`Initial Node Capacity: ${tree.totalNodeCount()}`); // 実際のノードは 0

  const style = new Style({
    display: Display.Flex,
    size: { width: 200, height: 40 },
    alignItems: AlignItems.Center,
    justifyContent: JustifyContent.Center,
  });

  const root = tree.newLeaf(style);
  tree.computeLayout(root, { width: 200, height: 40 });

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("i18n_ja-JP_getting-started_configuration example 2", async () => {
  const tree = new TaffyTree();

  // 合計 101px になる 2 つのアイテムを作成
  // 50.5 + 50.5 = 101
  const style = new Style({
    size: { width: 50.5, height: 50 },
    display: Display.Flex,
    justifyContent: JustifyContent.Center,
    alignItems: AlignItems.Center,
  });
  const child1 = tree.newLeaf(style);
  const child2 = tree.newLeaf(style);

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    size: { width: 150, height: 60 },
    alignItems: AlignItems.Center,
  });
  const root = tree.newWithChildren(rootStyle, [child1, child2]);

  // 1. デフォルト（丸め有効）
  tree.computeLayout(root, { width: 150, height: 60 });
  let layout1 = tree.getLayout(child1);
  // layout1.width はアルゴリズムに応じて 51 または 50 に丸められる場合があります

  // 2. 丸めを無効化
  tree.disableRounding();
  tree.computeLayout(root, { width: 150, height: 60 });
  layout1 = tree.getLayout(child1);
  // layout1.width は正確に 50.5 になります

  console.log(`Precise Width: ${layout1.width}`);

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("i18n_ja-JP_getting-started_configuration example 3", async () => {
  const tree = new TaffyTree();

  // ... ツリーを使用 ...

  // オプション 1：ツリーを再利用（推奨）
  // すべてのノードをクリアしますが、割り当てられたメモリは保持されます
  tree.clear();

  // オプション 2：完全に解放
  tree.free();
});
