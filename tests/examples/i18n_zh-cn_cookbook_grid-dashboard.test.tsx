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

test("i18n_zh-CN_cookbook_grid-dashboard example 1", async () => {
  const tree = new TaffyTree();

  const rootStyle = new Style({
    display: Display.Grid,
    size: { width: 600, height: 320 },
    gap: { width: 12, height: 12 },

    // 列：Nav (1fr)，Main (3fr)
    gridTemplateColumns: [
      { type: "Flex", value: 1 },
      { type: "Flex", value: 3 },
    ],
    // 行：Header (60px)，Content (1fr)
    gridTemplateRows: [
      { type: "Length", value: 60 },
      { type: "Flex", value: 1 },
    ],
  });

  // 创建子节点
  const header = tree.newLeaf(
    new Style({
      gridColumn: { start: 1, end: { span: 2 } }, // 跨越两列
      gridRow: { start: 1, end: { span: 1 } },
    }),
  );

  const nav = tree.newLeaf(
    new Style({
      gridColumn: { start: 1, end: { span: 1 } },
      gridRow: { start: 2, end: { span: 1 } },
    }),
  );

  const main = tree.newLeaf(
    new Style({
      gridColumn: { start: 2, end: { span: 1 } },
      gridRow: { start: 2, end: { span: 1 } },
    }),
  );

  const root = tree.newWithChildren(rootStyle, [header, nav, main]);

  tree.computeLayout(root, { width: 600, height: 320 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
