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

test("advanced_performance example 1", async () => {
  const tree = TaffyTree.withCapacity(2000);
  console.log(tree.totalNodeCount());

  const containerStyle = new Style();
  containerStyle.display = Display.Flex;
  containerStyle.flexDirection = FlexDirection.Row;
  containerStyle.size = { width: 220, height: 70 };
  containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

  const itemStyle = new Style();
  itemStyle.flexGrow = 1;

  const first = tree.newLeaf(itemStyle);
  const second = tree.newLeaf(itemStyle);
  const root = tree.newWithChildren(containerStyle, [first, second]);

  tree.computeLayout(root, { width: 220, height: 70 });

  return (
    <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
      <TaffyTreePreview tree={tree} root={root} />
      <div
        style={{
          padding: "8px 12px",
          borderRadius: 8,
          background: "rgba(0, 122, 255, 0.12)",
          color: "#0f172a",
          fontSize: 12,
          fontWeight: 600,
          display: "inline-flex",
          alignItems: "center",
        }}
      >
        Capacity: {tree.totalNodeCount()}
      </div>
    </div>
  );
});

test("advanced_performance example 2", async () => {
  const tree = new TaffyTree();

  const childStyle = new Style();
  childStyle.size = { width: 120, height: 60 };
  const child = tree.newLeaf(childStyle);
  const root = tree.newWithChildren(
    new Style({
      display: Display.Flex,
      padding: { left: 10, right: 10, top: 10, bottom: 10 },
    }),
    [child],
  );

  tree.computeLayout(root, { width: 200, height: 100 });

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("advanced_performance example 3", async () => {
  // ✅ Good
  const tree = new TaffyTree();
  const ITEM_STYLE = new Style({ flexGrow: 1 });
  for (let i = 0; i < 1000; i++) {
    tree.newLeaf(ITEM_STYLE);
  }
});

test("advanced_performance example 4", async () => {
  // ❌ Avoid if items are identical
  const tree = new TaffyTree();
  for (let i = 0; i < 1000; i++) {
    tree.newLeaf(new Style({ flexGrow: 1 }));
  }
});

test("advanced_performance example 5", async () => {
  const style = new Style();
  // ❌ Multiple calls = High overhead
  style.display = Display.Flex;
  style.width = 100;
  style.height = 100;
});

test("advanced_performance example 6", async () => {
  const style = new Style();
  // ✅ Single call = Low overhead
  style.set({
    display: Display.Flex,
    width: 100,
    height: 100,
  });
});

test("advanced_performance example 7", async () => {
  const tree = new TaffyTree();
  const node = tree.newLeaf(new Style());
  tree.computeLayout(node, { width: 100, height: 100 });

  // ❌ Multiple calls
  const layout = tree.getLayout(node);
  const x = layout.x;
  const y = layout.y;
  const w = layout.width;
  const h = layout.height;
});

test("advanced_performance example 8", async () => {
  const tree = new TaffyTree();
  const node = tree.newLeaf(new Style());
  tree.computeLayout(node, { width: 100, height: 100 });

  // ✅ Single call returns array of values
  const layout = tree.getLayout(node);
  const [x, y, w, h] = layout.get("x", "y", "width", "height");
});

test("advanced_performance example 9", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());

  const start = performance.now();
  tree.computeLayout(root, { width: 1000, height: 1000 });
  const end = performance.now();
  console.log(`Layout took ${end - start}ms`);
});
