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

test("i18n_zh-CN_styling_align-content example 1", async () => {
  const tree = new TaffyTree();

  const itemStyle = new Style({
    size: { width: 80, height: 30 },
    margin: { bottom: 5 },
  });

  // 创建足够的子元素以强制换行
  const children = Array.from({ length: 5 }).map(() => tree.newLeaf(itemStyle));

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    flexWrap: FlexWrap.Wrap, // alignContent 生效所必需
    size: { width: 200, height: 200 }, // 必须有额外的垂直空间

    // 修改这里以测试不同的值
    alignContent: AlignContent.Center,
  });

  const root = tree.newWithChildren(rootStyle, children);

  tree.computeLayout(root, { width: 200, height: 200 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
