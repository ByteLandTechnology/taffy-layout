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

test("i18n_ja-JP_styling_position example 1", async () => {
  const tree = new TaffyTree();

  const rootStyle = new Style({
    display: Display.Flex,
    size: { width: 300, height: 120 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    gap: { width: 10, height: 0 },
  });

  const standardItem = new Style({
    size: { width: 60, height: 60 },
    display: Display.Flex,
    justifyContent: JustifyContent.Center,
    alignItems: AlignItems.Center,
  });

  const absoluteItem = new Style({
    position: Position.Absolute,
    size: { width: 40, height: 40 },
    inset: { top: 0, right: 0, left: "auto", bottom: "auto" },
  });

  const child1 = tree.newLeaf(standardItem);
  const child2 = tree.newLeaf(standardItem);

  // この子要素は他の要素の上に浮かびます
  const childAbs = tree.newLeaf(absoluteItem);

  const root = tree.newWithChildren(rootStyle, [child1, child2, childAbs]);

  tree.computeLayout(root, { width: 300, height: 120 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
