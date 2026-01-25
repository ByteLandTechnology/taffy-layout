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

test("i18n_ja-JP_styling_size example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style();
  style.size = { width: 200, height: 100 };
  style.minSize = { width: 120, height: 60 };
  style.maxSize = { width: 300, height: 160 };

  const child = tree.newLeaf(style);

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.flexDirection = FlexDirection.Row;
  rootStyle.size = { width: 240, height: 140 };
  rootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

  const root = tree.newWithChildren(rootStyle, [child]);

  tree.computeLayout(root, {
    width: 240,
    height: 140,
  });

  console.log(tree.printTree(root));

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("i18n_ja-JP_styling_size example 2", async () => {
  const tree = new TaffyTree();

  const style = new Style();
  style.size = { width: "50%", height: "100%" };
  const child = tree.newLeaf(style);

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.flexDirection = FlexDirection.Row;
  rootStyle.size = { width: 200, height: 60 };

  const root = tree.newWithChildren(rootStyle, [child]);

  tree.computeLayout(root, {
    width: 200,
    height: 60,
  });

  console.log(tree.printTree(root));

  return <TaffyTreePreview tree={tree} root={root} />;
});
