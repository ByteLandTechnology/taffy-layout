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

test("styling_flex-direction example 1", async () => {
  const tree = new TaffyTree();

  const childStyle = new Style({
    size: { width: 50, height: 40 },
    margin: { left: 4, right: 4, top: 4, bottom: 4 },
  });

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    size: { width: 200, height: 160 },
    padding: { left: 8, right: 8, top: 8, bottom: 8 },
  });

  const root = tree.newWithChildren(rootStyle, [
    tree.newLeaf(childStyle),
    tree.newLeaf(childStyle),
    tree.newLeaf(childStyle),
  ]);

  tree.computeLayout(root, { width: 200, height: 160 });

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("styling_flex-direction example 2", async () => {
  const tree = new TaffyTree();

  const childStyle = new Style({
    size: { width: 50, height: 40 },
    margin: { left: 4, right: 4, top: 4, bottom: 4 },
  });

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.RowReverse,
    size: { width: 200, height: 160 },
    padding: { left: 8, right: 8, top: 8, bottom: 8 },
  });

  const root = tree.newWithChildren(rootStyle, [
    tree.newLeaf(childStyle),
    tree.newLeaf(childStyle),
    tree.newLeaf(childStyle),
  ]);

  tree.computeLayout(root, { width: 200, height: 160 });

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("styling_flex-direction example 3", async () => {
  const tree = new TaffyTree();

  const childStyle = new Style({
    size: { width: 50, height: 40 },
    margin: { left: 4, right: 4, top: 4, bottom: 4 },
  });

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    size: { width: 200, height: 160 },
    padding: { left: 8, right: 8, top: 8, bottom: 8 },
  });

  const root = tree.newWithChildren(rootStyle, [
    tree.newLeaf(childStyle),
    tree.newLeaf(childStyle),
    tree.newLeaf(childStyle),
  ]);

  tree.computeLayout(root, { width: 200, height: 160 });

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("styling_flex-direction example 4", async () => {
  const tree = new TaffyTree();

  const childStyle = new Style({
    size: { width: 50, height: 40 },
    margin: { left: 4, right: 4, top: 4, bottom: 4 },
  });

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.ColumnReverse,
    size: { width: 200, height: 160 },
    padding: { left: 8, right: 8, top: 8, bottom: 8 },
  });

  const root = tree.newWithChildren(rootStyle, [
    tree.newLeaf(childStyle),
    tree.newLeaf(childStyle),
    tree.newLeaf(childStyle),
  ]);

  tree.computeLayout(root, { width: 200, height: 160 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
