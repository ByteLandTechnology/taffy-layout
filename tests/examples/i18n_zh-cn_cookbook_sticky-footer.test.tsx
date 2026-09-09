import React from "react";
import { test } from "vitest";
import init, {
  TaffyTree,
  Style,
  // Add all other exports that might be needed
  Display,
  Direction,
  Float,
  Clear,
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
  MinTrackSizingFunction,
  MaxTrackSizingFunction,
  GridTemplateArea,
  GridTemplateComponent,
  GridTemplateRepetition,
  RepetitionCount,
  StyleProperty,
  StylePropertyValues,
  LayoutProperty,
  Line,
  Point,
  TaffyError,
  Layout,
  MeasureFunction,
} from "taffy-layout";

// Global init for the suite
await init();

// Mock TaffyTreePreview component
const TaffyTreePreview = (_props: any) => null;

test("i18n_zh-CN_cookbook_sticky-footer example 1", async () => {
  const tree = new TaffyTree();

  // 页面容器
  const pageStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    size: { width: 300, height: "auto" },
    minHeight: 300, // 内容较少时至少填满视口，内容增多时允许页面增高
  });

  const header = tree.newLeaf(
    new Style({ size: { width: "100%", height: 50 }, marginBottom: 10 }),
  );
  const footer = tree.newLeaf(
    new Style({ size: { width: "100%", height: 50 }, marginTop: 10 }),
  );

  // 内容增长以填充空间
  const content = tree.newLeaf(
    new Style({
      flexGrow: 1,
      size: { width: "100%", height: "auto" },
    }),
  );

  const root = tree.newWithChildren(pageStyle, [header, content, footer]);

  tree.computeLayout(root, { width: 300, height: 300 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
