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

test("i18n_zh-CN_getting-started_quick-start example 1", async () => {
  // 1. 初始化库
  const tree = new TaffyTree();

  // 2. 创建样式
  const containerStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    alignItems: AlignItems.Center,
    size: { width: 300, height: 200 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });

  const childStyle = new Style({
    flexGrow: 1,
    size: { width: "100%", height: "auto" },
  });

  // 3. 创建节点树
  //    （如果使用 newWithChildren，必须先创建叶子节点）
  const child1 = tree.newLeaf(childStyle);
  const child2 = tree.newLeaf(childStyle);
  const container = tree.newWithChildren(containerStyle, [child1, child2]);

  // 4. 计算布局
  //    传入根节点和可用空间
  tree.computeLayout(container, { width: 300, height: 200 });

  // 5. 读取计算结果
  const containerLayout = tree.getLayout(container);
  const child1Layout = tree.getLayout(child1);

  console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
  // 输出: Container: 300x200

  // 6. 调试：打印整个树结构
  console.log(tree.printTree(container));
});
