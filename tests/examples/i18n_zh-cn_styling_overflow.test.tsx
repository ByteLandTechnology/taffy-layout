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

test("i18n_zh-CN_styling_overflow example 1", async () => {
  const tree = new TaffyTree();
  const style = new Style({
    overflow: { x: Overflow.Scroll, y: Overflow.Scroll },
    scrollbarWidth: 15, // 用于设置预估滚动条尺寸的辅助选项
  });

  // 布局计算后：
  const node = tree.newLeaf(style);
  tree.computeLayout(node, { width: 100, height: 100 });
  const layout = tree.getLayout(node);
  console.log(
    `Scrollbar Size: ${layout.scrollbarWidth} x ${layout.scrollbarHeight}`,
  );
});

test("i18n_zh-CN_styling_overflow example 2", async () => {
  const tree = new TaffyTree();

  const container = tree.newLeaf(
    new Style({
      size: { width: 100, height: 100 },
      padding: { left: 10, right: 10, top: 10, bottom: 10 },
      // 尝试将其改为 Hidden
      overflow: { x: Overflow.Visible, y: Overflow.Visible },
    }),
  );

  const bigContent = tree.newLeaf(
    new Style({
      size: { width: 200, height: 200 },
    }),
  );

  tree.addChild(container, bigContent);

  tree.computeLayout(container, { width: 100, height: 100 });

  return <TaffyTreePreview tree={tree} root={container} />;
});
