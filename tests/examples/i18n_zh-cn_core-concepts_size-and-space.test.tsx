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

test("i18n_zh-CN_core-concepts_size-and-space example 1", async () => {
  // 固定
  const fixedTree = new TaffyTree();
  const fixedStyle = new Style();
  fixedStyle.size = { width: 120, height: 40 };
  const fixedChild = fixedTree.newLeaf(fixedStyle);

  const fixedRootStyle = new Style();
  fixedRootStyle.display = Display.Flex;
  fixedRootStyle.flexDirection = FlexDirection.Row;
  fixedRootStyle.size = { width: 200, height: 80 };
  fixedRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

  const fixedRoot = fixedTree.newWithChildren(fixedRootStyle, [fixedChild]);

  fixedTree.computeLayout(fixedRoot, { width: 200, height: 80 });
  console.log(fixedTree.printTree(fixedRoot));

  // 弹性
  const flexibleTree = new TaffyTree();
  const flexibleStyle = new Style();
  flexibleStyle.size = { width: "auto", height: 40 };
  const flexibleChild = flexibleTree.newLeaf(flexibleStyle);

  const flexibleRootStyle = new Style();
  flexibleRootStyle.display = Display.Flex;
  flexibleRootStyle.flexDirection = FlexDirection.Row;
  flexibleRootStyle.size = { width: 200, height: 80 };
  flexibleRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

  const flexibleRoot = flexibleTree.newWithChildren(flexibleRootStyle, [
    flexibleChild,
  ]);

  flexibleTree.computeLayout(flexibleRoot, {
    width: "max-content",
    height: 80,
  });
  console.log(flexibleTree.printTree(flexibleRoot));

  return (
    <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
      <div>
        <div style={{ marginBottom: 6, fontSize: 12 }}>固定空间</div>
        <TaffyTreePreview tree={fixedTree} root={fixedRoot} />
      </div>
      <div>
        <div style={{ marginBottom: 6, fontSize: 12 }}>最大内容宽度</div>
        <TaffyTreePreview tree={flexibleTree} root={flexibleRoot} />
      </div>
    </div>
  );
});

test("i18n_zh-CN_core-concepts_size-and-space example 2", async () => {
  const tree = new TaffyTree();

  const style = new Style();
  style.size = { width: "50%", height: "100%" };
  const child = tree.newLeaf(style);

  const rootStyle = new Style();
  rootStyle.size = { width: 260, height: 160 };
  rootStyle.padding = { left: 16, right: 16, top: 16, bottom: 16 };

  const root = tree.newWithChildren(rootStyle, [child]);

  tree.computeLayout(root, {
    width: 260,
    height: 160,
  });

  console.log(tree.printTree(root));

  return <TaffyTreePreview tree={tree} root={root} />;
});
