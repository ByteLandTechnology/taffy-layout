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

test("i18n_ja-JP_getting-started_building-trees example 1", async () => {
  const tree = new TaffyTree();

  // 最初にスタイルを作成
  const style = new Style({
    display: Display.Flex,
    size: { width: 100, height: 50 },
  });

  // ノードを作成
  const leafNode = tree.newLeaf(style);
});

test("i18n_ja-JP_getting-started_building-trees example 2", async () => {
  const tree = new TaffyTree();
  const containerStyle = new Style({ display: Display.Flex });

  const child1 = tree.newLeaf(new Style());
  const child2 = tree.newLeaf(new Style());

  // 子ノードをすぐに含めてコンテナを作成
  const containerNode = tree.newWithChildren(containerStyle, [child1, child2]);
});

test("i18n_ja-JP_getting-started_building-trees example 3", async () => {
  const tree = new TaffyTree();
  const parent = tree.newLeaf(new Style());
  const child = tree.newLeaf(new Style());

  // リストの末尾に子ノードを追加
  tree.addChild(parent, child); // インデックス: 0

  // 先頭に新しい子ノードを挿入
  const firstChild = tree.newLeaf(new Style());
  tree.insertChildAtIndex(parent, 0, firstChild); // インデックス: 0、以前の子ノードはインデックス 1 に

  // 子ノードを置換
  const newChild = tree.newLeaf(new Style());
  tree.replaceChildAtIndex(parent, 1, newChild);

  // 子ノードを削除
  tree.removeChild(parent, firstChild);
});

test("i18n_ja-JP_getting-started_building-trees example 4", async () => {
  const tree = new TaffyTree();
  const node = tree.newLeaf(new Style({ flexGrow: 1 }));

  // 後でスタイルを更新
  const newStyle = new Style({ flexGrow: 2, width: 100 });
  tree.setStyle(node, newStyle);
});
