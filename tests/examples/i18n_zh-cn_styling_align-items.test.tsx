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

test("i18n_zh-CN_styling_align-items example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style({
    display: Display.Flex,
    size: { width: 50, height: 30 },
    alignItems: AlignItems.Center,
    justifyContent: JustifyContent.Center,
  });

  const labelStyle = new Style({
    flexGrow: 1,
  });

  // 创建不同高度的子元素以演示对齐效果
  const child1 = tree.newLeaf(new Style({ size: { width: 40, height: 20 } }));
  const child2 = tree.newLeaf(new Style({ size: { width: 40, height: 40 } }));
  const child3 = tree.newLeaf(new Style({ size: { width: 40, height: 60 } }));

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    size: { width: 300, height: 100 },
    gap: { width: 10, height: 0 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },

    // 修改这里以测试不同的值
    alignItems: AlignItems.Center,
    // 选项: FlexStart, FlexEnd, Stretch, Baseline
  });

  const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

  tree.computeLayout(root, { width: 300, height: 100 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
