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

test("i18n_zh-CN_styling_margin-padding-border example 1", async () => {
  const tree = new TaffyTree();

  // 内部内容
  const contentNode = tree.newLeaf(
    new Style({
      flexGrow: 1, // 填充可用空间
    }),
  );

  // 演示盒模型的容器
  const boxNode = tree.newWithChildren(
    new Style({
      size: { width: 200, height: 120 },
      display: Display.Flex,

      // 1. Margin（外部）
      margin: { left: 20, top: 20 },

      // 2. Border（边界）- 仅逻辑宽度
      border: { left: 5, right: 5, top: 5, bottom: 5 },

      // 3. Padding（内部）
      padding: { left: 15, right: 15, top: 15, bottom: 15 },
    }),
    [contentNode],
  );

  // 保存示例的根节点
  const root = tree.newWithChildren(
    new Style({
      size: { width: 300, height: 200 },
    }),
    [boxNode],
  );

  tree.computeLayout(root, { width: 300, height: 200 });

  // 可视化层次结构
  console.log(tree.printTree(root));

  return <TaffyTreePreview tree={tree} root={root} />;
});
