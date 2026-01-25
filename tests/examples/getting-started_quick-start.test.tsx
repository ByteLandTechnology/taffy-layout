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

test("getting-started_quick-start example 1", async () => {
  // 1. Initialize the library
  const tree = new TaffyTree();

  // 2. Create styles
  const containerStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    alignItems: AlignItems.Center,
    size: { width: 300, height: 200 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });

  const childStyle = new Style({
    flexGrow: 1,
    size: { width: "100%", height: "auto" },
  });

  // 3. Create the node tree
  //    (Leaf nodes must be created before parents if using newWithChildren)
  const child1 = tree.newLeaf(childStyle);
  const child2 = tree.newLeaf(childStyle);
  const container = tree.newWithChildren(containerStyle, [child1, child2]);

  // 4. Compute layout
  //    Pass the root node and the available space
  tree.computeLayout(container, { width: 300, height: 200 });

  // 5. Read computed results
  const containerLayout = tree.getLayout(container);
  const child1Layout = tree.getLayout(child1);

  console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
  // Output: Container: 300x200

  // 6. Debugging: Print the whole tree structure
  console.log(tree.printTree(container));
});
