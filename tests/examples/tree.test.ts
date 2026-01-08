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

test("tree example 1", async () => {
  const tree = new TaffyTree();

  // Container style with explicit types
  const containerStyle = new Style();
  containerStyle.display = Display.Flex;
  containerStyle.flexDirection = FlexDirection.Column;

  const containerSize: Size<Dimension> = {
    width: 300,
    height: 200,
  };
  containerStyle.size = containerSize;

  // Child style
  const childStyle = new Style();
  childStyle.flexGrow = 1;

  // Create nodes with type annotations
  const child1: bigint = tree.newLeaf(childStyle);
  const child2: bigint = tree.newLeaf(childStyle);
  const container: bigint = tree.newWithChildren(
    containerStyle,
    BigUint64Array.from([child1, child2]),
  );

  // Compute layout with typed available space
  const availableSpace: Size<AvailableSpace> = {
    width: 300,
    height: 200,
  };
  tree.computeLayout(container, availableSpace);

  // Get typed layout
  const layout = tree.getLayout(child1);
  console.log(`Size: ${layout.width}x${layout.height}`);
});

test("tree example 2", async () => {
  try {
    const tree = new TaffyTree();
    const style = new Style();
    const nodeId = tree.newLeaf(style);
    console.log("Created node:", nodeId);
  } catch (e) {
    if (e instanceof TaffyError) {
      console.error("Error:", e.message);
    }
  }
});

test("tree example 3", async () => {
  const tree: TaffyTree = new TaffyTree();
});

test("tree example 4", async () => {
  const tree: TaffyTree = TaffyTree.withCapacity(1000);
});

test("tree example 5", async () => {
  const tree = new TaffyTree();
  tree.enableRounding();
});

test("tree example 6", async () => {
  const tree = new TaffyTree();
  const node = tree.newLeaf(new Style());
  tree.disableRounding();
  const layout = tree.getLayout(node);
  console.log(layout.x);
});

test("tree example 7", async () => {
  const tree = new TaffyTree();
  const style = new Style();
  style.size = { width: 100, height: 50 };
  const nodeId: bigint = tree.newLeaf(style);
});

test("tree example 8", async () => {
  interface TextContext {
    text: string;
    isBold: boolean;
  }

  const tree = new TaffyTree();
  const style = new Style();
  const context: TextContext = { text: "Hello, World!", isBold: true };
  const nodeId: bigint = tree.newLeafWithContext(style, context);
});

test("tree example 9", async () => {
  const tree = new TaffyTree();
  const containerStyle = new Style();
  containerStyle.display = Display.Flex;

  const child1: bigint = tree.newLeaf(new Style());
  const child2: bigint = tree.newLeaf(new Style());

  const container: bigint = tree.newWithChildren(
    containerStyle,
    BigUint64Array.from([child1, child2]),
  );
});

test("tree example 10", async () => {
  const tree = new TaffyTree();
  tree.clear();
  console.log(tree.totalNodeCount());
});

test("tree example 11", async () => {
  const tree = new TaffyTree();
  const nodeId = tree.newLeaf(new Style());
  try {
    const removedId: bigint = tree.remove(nodeId);
  } catch (e) {
    console.error("Node doesn't exist");
  }
});

test("tree example 12", async () => {
  const tree = new TaffyTree();
  const nodeId = tree.newLeaf(new Style());
  interface Context {
    text: string;
  }
  tree.setNodeContext(nodeId, { text: "Updated text" } as Context);
});

test("tree example 13", async () => {
  const tree = new TaffyTree();
  const nodeId = tree.newLeaf(new Style());
  interface Context {
    text: string;
  }
  const context = tree.getNodeContext(nodeId) as Context | undefined;
  if (context) {
    console.log(context.text);
  }
});

test("tree example 14", async () => {
  const tree = new TaffyTree();
  const id1 = tree.newLeaf(new Style());
  const id2 = tree.newLeaf(new Style());
  const nodes = BigUint64Array.from([id1, id2]);
  const contexts = tree.getDisjointNodeContextMut(nodes);
});

test("tree example 15", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const childId = tree.newLeaf(new Style());
  tree.addChild(parentId, childId);
});

test("tree example 16", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const childId = tree.newLeaf(new Style());
  tree.insertChildAtIndex(parentId, 0, childId);
});

test("tree example 17", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const child1 = tree.newLeaf(new Style());
  const child2 = tree.newLeaf(new Style());
  const child3 = tree.newLeaf(new Style());
  const children = BigUint64Array.from([child1, child2, child3]);
  tree.setChildren(parentId, children);
});

test("tree example 18", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const childId = tree.newLeaf(new Style());
  tree.addChild(parentId, childId);
  tree.removeChild(parentId, childId);
});

test("tree example 19", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const childId = tree.newLeaf(new Style());
  tree.addChild(parentId, childId);
  const removedId: bigint = tree.removeChildAtIndex(parentId, 0);
});

test("tree example 20", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const oldChild = tree.newLeaf(new Style());
  const newChildId = tree.newLeaf(new Style());
  tree.addChild(parentId, oldChild);
  const child = tree.newLeaf(new Style()); // filler child at index 0 if needed, but index 1 implies 2 children
  tree.insertChildAtIndex(parentId, 0, child);

  const oldChildId: bigint = tree.replaceChildAtIndex(parentId, 1, newChildId);
});

test("tree example 21", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const childId = tree.newLeaf(new Style());
  tree.addChild(parentId, childId);
  const firstChild: bigint = tree.getChildAtIndex(parentId, 0);
});

test("tree example 22", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const child1 = tree.newLeaf(new Style());
  const child2 = tree.newLeaf(new Style());
  const child3 = tree.newLeaf(new Style());
  tree.setChildren(parentId, BigUint64Array.from([child1, child2, child3]));

  tree.removeChildrenRange(parentId, 1, 3);
});

test("tree example 23", async () => {
  const tree = new TaffyTree();
  const count: number = tree.totalNodeCount();
});

test("tree example 24", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const count: number = tree.childCount(parentId);
});

test("tree example 25", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const childId = tree.newLeaf(new Style());
  tree.addChild(parentId, childId);
  const parent: bigint | undefined = tree.parent(childId);
});

test("tree example 26", async () => {
  const tree = new TaffyTree();
  const parentId = tree.newLeaf(new Style());
  const children: BigUint64Array = tree.children(parentId);
});

test("tree example 27", async () => {
  const tree = new TaffyTree();
  const nodeId = tree.newLeaf(new Style());
  const newStyle = new Style();
  newStyle.flexGrow = 2;
  tree.setStyle(nodeId, newStyle);
});

test("tree example 28", async () => {
  const tree = new TaffyTree();
  const nodeId = tree.newLeaf(new Style());
  const style: Style = tree.getStyle(nodeId);
  console.log("Flex grow:", style.flexGrow);
});

test("tree example 29", async () => {
  const tree = new TaffyTree();
  const style = new Style();
  style.size = { width: 100, height: 100 };
  const rootId = tree.newLeaf(style);
  const nodeId = rootId;

  tree.computeLayout(rootId, { width: 800, height: 600 });
  const layout: Layout = tree.getLayout(nodeId);
  console.log(
    `Position: (${layout.x}, ${layout.y}), Size: ${layout.width}x${layout.height}`,
  );
});

test("tree example 30", async () => {
  const tree = new TaffyTree();
  const nodeId = tree.newLeaf(new Style());
  const layout: Layout = tree.unroundedLayout(nodeId);
  console.log(`Exact width: ${layout.width}`);
});

test("tree example 31", async () => {
  const tree = new TaffyTree();
  const rootId = tree.newLeaf(new Style());
  const nodeId = rootId;
  const availableSpace = { width: 100, height: 100 };

  // After updating text content
  tree.setNodeContext(nodeId, { text: "Updated text" });
  tree.markDirty(nodeId);
  tree.computeLayout(rootId, availableSpace);
});

test("tree example 32", async () => {
  const tree = new TaffyTree();
  const rootId = tree.newLeaf(new Style());
  const nodeId = rootId;
  const availableSpace = { width: 100, height: 100 };

  if (tree.dirty(nodeId)) {
    tree.computeLayout(rootId, availableSpace);
  }
});

test("tree example 33", async () => {
  const tree = new TaffyTree();
  const rootId = tree.newLeaf(new Style());

  const measureText = (text: string, width: number) => ({
    width: 0,
    height: 0,
  });

  tree.computeLayoutWithMeasure(
    rootId,
    { width: 800, height: "max-content" },
    (known, available, node, context, style) => {
      if (context?.text) {
        const measured = measureText(context.text, available.width as number);
        return { width: measured.width, height: measured.height };
      }
      return { width: 0, height: 0 };
    },
  );
});

test("tree example 34", async () => {
  const tree = new TaffyTree();
  const rootId = tree.newLeaf(new Style());

  // Fixed size container
  tree.computeLayout(rootId, { width: 800, height: 600 });

  // Flexible width, fixed height
  tree.computeLayout(rootId, { width: "max-content", height: 600 });

  // Minimum content size
  tree.computeLayout(rootId, { width: "min-content", height: "min-content" });
});

test("tree example 35", async () => {
  const tree = new TaffyTree();
  const rootId = tree.newLeaf(new Style());
  tree.computeLayout(rootId, { width: 800, height: 600 });
});

test("tree example 36", async () => {
  const tree = new TaffyTree();
  const rootId = tree.newLeaf(new Style());
  tree.printTree(rootId);
  // Output appears in browser console
});
