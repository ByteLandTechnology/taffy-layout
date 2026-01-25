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

test("i18n_zh-CN_getting-started_building-trees example 1", async () => {
  const tree = new TaffyTree();

  // 先创建样式
  const style = new Style({
    display: Display.Flex,
    size: { width: 100, height: 50 },
  });

  // 创建节点
  const leafNode = tree.newLeaf(style);
});

test("i18n_zh-CN_getting-started_building-trees example 2", async () => {
  const tree = new TaffyTree();
  const containerStyle = new Style({ display: Display.Flex });

  const child1 = tree.newLeaf(new Style());
  const child2 = tree.newLeaf(new Style());

  // 创建容器并立即附加子节点
  const containerNode = tree.newWithChildren(containerStyle, [child1, child2]);
});

test("i18n_zh-CN_getting-started_building-trees example 3", async () => {
  const tree = new TaffyTree();
  const parent = tree.newLeaf(new Style());
  const child = tree.newLeaf(new Style());

  // 在列表末尾添加子节点
  tree.addChild(parent, child); // 索引: 0

  // 在开头插入新子节点
  const firstChild = tree.newLeaf(new Style());
  tree.insertChildAtIndex(parent, 0, firstChild); // 索引: 0，之前的子节点变为索引 1

  // 替换子节点
  const newChild = tree.newLeaf(new Style());
  tree.replaceChildAtIndex(parent, 1, newChild);

  // 删除子节点
  tree.removeChild(parent, firstChild);
});

test("i18n_zh-CN_getting-started_building-trees example 4", async () => {
  const tree = new TaffyTree();
  const node = tree.newLeaf(new Style({ flexGrow: 1 }));

  // 稍后更新样式
  const newStyle = new Style({ flexGrow: 2, width: 100 });
  tree.setStyle(node, newStyle);
});
