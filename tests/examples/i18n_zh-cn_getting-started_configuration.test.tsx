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

test("i18n_zh-CN_getting-started_configuration example 1", async () => {
  // 初始化为可容纳 1,000 个节点
  const tree = TaffyTree.withCapacity(1000);
  console.log(`Initial Node Capacity: ${tree.totalNodeCount()}`); // 0 个实际节点

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

test("i18n_zh-CN_getting-started_configuration example 2", async () => {
  const tree = new TaffyTree();

  // 创建两个总和为 101px 的项目
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

  // 1. 默认（启用舍入）
  tree.computeLayout(root, { width: 150, height: 60 });
  let layout1 = tree.getLayout(child1);
  // layout1.width 可能根据算法舍入为 51 或 50

  // 2. 禁用舍入
  tree.disableRounding();
  tree.computeLayout(root, { width: 150, height: 60 });
  layout1 = tree.getLayout(child1);
  // layout1.width 将精确为 50.5

  console.log(`Precise Width: ${layout1.width}`);

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("i18n_zh-CN_getting-started_configuration example 3", async () => {
  const tree = new TaffyTree();

  // ... 使用树 ...

  // 选项 1：重用树（推荐）
  // 清除所有节点但保持已分配的内存
  tree.clear();

  // 选项 2：完全释放
  tree.free();
});
