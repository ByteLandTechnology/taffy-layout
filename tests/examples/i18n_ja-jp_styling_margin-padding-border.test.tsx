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

test("i18n_ja-JP_styling_margin-padding-border example 1", async () => {
  const tree = new TaffyTree();

  // 内部コンテンツ
  const contentNode = tree.newLeaf(
    new Style({
      flexGrow: 1, // 利用可能なスペースを埋める
    }),
  );

  // ボックスモデルをデモンストレーションするコンテナ
  const boxNode = tree.newWithChildren(
    new Style({
      size: { width: 200, height: 120 },
      display: Display.Flex,

      // 1. Margin（外側）
      margin: { left: 20, top: 20 },

      // 2. Border（境界）- 論理幅のみ
      border: { left: 5, right: 5, top: 5, bottom: 5 },

      // 3. Padding（内側）
      padding: { left: 15, right: 15, top: 15, bottom: 15 },
    }),
    [contentNode],
  );

  // 例を保持するルート
  const root = tree.newWithChildren(
    new Style({
      size: { width: 300, height: 200 },
    }),
    [boxNode],
  );

  tree.computeLayout(root, { width: 300, height: 200 });

  // 階層構造を可視化
  console.log(tree.printTree(root));

  return <TaffyTreePreview tree={tree} root={root} />;
});
