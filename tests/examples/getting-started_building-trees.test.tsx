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

test("getting-started_building-trees example 1", async () => {
  const tree = new TaffyTree();

  // Create styles first
  const style = new Style({
    display: Display.Flex,
    size: { width: 100, height: 50 },
  });

  // Create the node
  const leafNode = tree.newLeaf(style);
});

test("getting-started_building-trees example 2", async () => {
  const tree = new TaffyTree();
  const containerStyle = new Style({ display: Display.Flex });

  const child1 = tree.newLeaf(new Style());
  const child2 = tree.newLeaf(new Style());

  // Create container with children attached immediately
  const containerNode = tree.newWithChildren(containerStyle, [child1, child2]);
});

test("getting-started_building-trees example 3", async () => {
  const tree = new TaffyTree();
  const parent = tree.newLeaf(new Style());
  const child = tree.newLeaf(new Style());

  // Add a child to the end of the list
  tree.addChild(parent, child); // Index: 0

  // Insert a new child at the beginning
  const firstChild = tree.newLeaf(new Style());
  tree.insertChildAtIndex(parent, 0, firstChild); // Index: 0, previous child becomes Index 1

  // Replace a child
  const newChild = tree.newLeaf(new Style());
  tree.replaceChildAtIndex(parent, 1, newChild);

  // Remove a child
  tree.removeChild(parent, firstChild);
});

test("getting-started_building-trees example 4", async () => {
  const tree = new TaffyTree();
  const node = tree.newLeaf(new Style({ flexGrow: 1 }));

  // Update style later
  const newStyle = new Style({ flexGrow: 2, width: 100 });
  tree.setStyle(node, newStyle);
});
