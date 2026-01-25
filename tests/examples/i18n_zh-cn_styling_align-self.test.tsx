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

test("i18n_zh-CN_styling_align-self example 1", async () => {
  const tree = new TaffyTree();

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    alignItems: AlignItems.FlexStart, // 默认对齐方式为顶部
    size: { width: 300, height: 100 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    gap: { width: 10, height: 0 },
  });

  const standardItem = new Style({ size: { width: 50, height: 40 } });

  // 这个子元素覆盖了父级的 FlexStart 对齐方式
  const selfAlignedItem = new Style({
    size: { width: 50, height: 40 },
    alignSelf: AlignSelf.FlexEnd,
  });

  const child1 = tree.newLeaf(standardItem);
  const child2 = tree.newLeaf(selfAlignedItem); // 将显示在底部
  const child3 = tree.newLeaf(standardItem);

  const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

  tree.computeLayout(root, { width: 300, height: 100 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
