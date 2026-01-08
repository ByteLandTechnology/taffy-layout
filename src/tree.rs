//! # Layout Tree Management Module
//!
//! This module provides the [`JsTaffyTree`] struct, which is the main entry point for
//! creating and managing layout trees. It wraps the native Taffy layout engine and
//! exposes a JavaScript-friendly API.
//!
//! ## Overview
//!
//! The `TaffyTree` class manages a collection of nodes organized in a tree structure.
//! Each node has:
//! - A unique ID (represented as `bigint` in JavaScript)
//! - A style configuration that defines its layout properties
//! - Optional context data (any JavaScript value)
//! - Zero or more child nodes
//!
//! ## Workflow
//!
//! 1. Create a `TaffyTree` instance
//! 2. Create nodes using `newLeaf()` or `newWithChildren()`
//! 3. Build the tree structure using `addChild()`, `setChildren()`, etc.
//! 4. Compute layout using `computeLayout()` or `computeLayoutWithMeasure()`
//! 5. Retrieve computed layouts using `getLayout()`
//!
//!
//! @example
//! ```typescript
//! import init, {
//!   TaffyTree,
//!   Style,
//!   Display,
//!   FlexDirection,
//!   type Size,
//!   type AvailableSpace,
//!   type Dimension
//! } from 'taffy-layout';
//!
//! await init();
//!
//! const tree = new TaffyTree();
//!
//! // Container style with explicit types
//! const containerStyle = new Style();
//! containerStyle.display = Display.Flex;
//! containerStyle.flexDirection = FlexDirection.Column;
//!
//! const containerSize: Size<Dimension> = {
//!   width: 300,
//!   height: 200
//! };
//! containerStyle.size = containerSize;
//!
//! // Child style
//! const childStyle = new Style();
//! childStyle.flexGrow = 1;
//!
//! // Create nodes with type annotations
//! const child1: bigint = tree.newLeaf(childStyle);
//! const child2: bigint = tree.newLeaf(childStyle);
//! const container: bigint = tree.newWithChildren(
//!   containerStyle,
//!   BigUint64Array.from([child1, child2])
//! );
//!
//! // Compute layout with typed available space
//! const availableSpace: Size<AvailableSpace> = {
//!   width: 300,
//!   height: 200
//! };
//! tree.computeLayout(container, availableSpace);
//!
//! // Get typed layout
//! const layout = tree.getLayout(child1);
//! console.log(`Size: ${layout.width}x${layout.height}`);
//! ```
//!
//! ## Node IDs
//!
//! Node IDs are represented as `bigint` in JavaScript (u64 in Rust). They are stable
//! for the lifetime of the node and can be stored/compared as needed.
//!
//! ## Error Handling
//!
//! Methods that can fail throw a `TaffyError` as a JavaScript exception.
//!
//! @example
//! ```typescript
//! try {
//!   const tree = new TaffyTree();
//!   const style = new Style();
//!   const nodeId = tree.newLeaf(style);
//!   console.log('Created node:', nodeId);
//! } catch (e) {
//!   if (e instanceof TaffyError) {
//!     console.error('Error:', e.message);
//!   }
//! }
//! ```

use crate::error::{JsTaffyError, map_bool_result, map_node_result, map_void_result, to_js_error};
use crate::layout::JsLayout;
use crate::style::JsStyle;
use crate::types::{AvailableSizeDto, JsAvailableSizeArg, JsMeasureFunctionArg};
use crate::{DetailedGridInfoDto, DetailedGridItemsInfoDto, DetailedGridTracksInfoDto};

use taffy::TaffyError as NativeTaffyError;
use taffy::TaffyTree;
use taffy::prelude::*;
use taffy::style::{self as TaffyStyle};
#[cfg(feature = "detailed_layout_info")]
use taffy::tree::DetailedLayoutInfo;
use wasm_bindgen::prelude::*;

// =============================================================================
// TaffyTree Struct
// =============================================================================

/// The main layout tree class for creating nodes, computing layouts, and managing a tree of styled elements.
///
/// TaffyTree is the entry point for the Taffy layout engine. It manages
/// a tree of nodes and computes their layouts using CSS Flexbox and Grid algorithms.
///
#[wasm_bindgen(js_name = TaffyTree)]
pub struct JsTaffyTree {
    /// The underlying Taffy tree with JsValue context type
    tree: TaffyTree<JsValue>,
}

#[wasm_bindgen(js_class = "TaffyTree")]
impl JsTaffyTree {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a new empty TaffyTree
    ///
    /// The tree starts with no nodes. Use `newLeaf()` or `newWithChildren()`
    /// to add nodes.
    ///
    /// @example
    /// ```typescript
    /// const tree: TaffyTree = new TaffyTree();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsTaffyTree {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        JsTaffyTree {
            tree: TaffyTree::new(),
        }
    }

    /// Creates a new TaffyTree with pre-allocated capacity
    ///
    /// Use this when you know approximately how many nodes will be in the tree.
    /// This can improve performance by reducing memory reallocations.
    ///
    /// @param capacity - The number of nodes to pre-allocate space for
    ///
    /// @example
    /// ```typescript
    /// const tree: TaffyTree = TaffyTree.withCapacity(1000);
    /// ```
    #[wasm_bindgen(js_name = withCapacity)]
    pub fn with_capacity(capacity: usize) -> JsTaffyTree {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        JsTaffyTree {
            tree: TaffyTree::with_capacity(capacity),
        }
    }

    // =========================================================================
    // Configuration
    // =========================================================================

    /// Enables rounding of layout values to whole pixels
    ///
    /// When enabled (default), computed layout values like position and size
    /// are rounded to the nearest integer. This prevents sub-pixel rendering
    /// issues in most rendering contexts.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// tree.enableRounding();
    /// ```
    #[wasm_bindgen(js_name = enableRounding)]
    pub fn enable_rounding(&mut self) {
        self.tree.enable_rounding();
    }

    /// Disables rounding of layout values
    ///
    /// When disabled, computed layout values retain their fractional precision.
    /// Use this when you need sub-pixel accuracy or when performing custom
    /// rounding.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const node = tree.newLeaf(new Style());
    /// tree.disableRounding();
    /// const layout = tree.getLayout(node);
    /// console.log(layout.x);
    /// ```
    #[wasm_bindgen(js_name = disableRounding)]
    pub fn disable_rounding(&mut self) {
        self.tree.disable_rounding();
    }

    // =========================================================================
    // Node Creation
    // =========================================================================

    /// Creates a new leaf node with the given style
    ///
    /// A leaf node has no children. Use this for elements that contain
    /// content (like text) rather than other elements.
    ///
    /// @param style - The style configuration for the node
    /// @returns - The node ID (`bigint`)
    /// @throws `TaffyError` if the node cannot be created
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const style = new Style();
    /// style.size = { width: 100, height: 50 };
    /// const nodeId: bigint = tree.newLeaf(style);
    /// ```
    #[wasm_bindgen(js_name = newLeaf)]
    pub fn new_leaf(&mut self, style: &JsStyle) -> Result<u64, JsValue> {
        map_node_result(self.tree.new_leaf(style.inner.clone()))
    }

    /// Creates a new leaf node with an attached context value
    ///
    /// The context can be any JavaScript value and is passed to the measure
    /// function during layout computation. This is useful for storing
    /// references to text content or other dynamic data.
    ///
    /// @param style - The style configuration for the node
    /// @param context - Any JavaScript value to attach to the node
    /// @returns - The node ID (`bigint`)
    /// @throws `TaffyError` if the node cannot be created
    ///
    /// @example
    /// ```typescript
    /// interface TextContext { text: string; isBold: boolean; }
    ///
    /// const tree = new TaffyTree();
    /// const style = new Style();
    /// const context: TextContext = { text: "Hello, World!", isBold: true };
    /// const nodeId: bigint = tree.newLeafWithContext(style, context);
    /// ```
    #[wasm_bindgen(js_name = newLeafWithContext)]
    pub fn new_leaf_with_context(
        &mut self,
        style: &JsStyle,
        context: JsValue,
    ) -> Result<u64, JsValue> {
        map_node_result(
            self.tree
                .new_leaf_with_context(style.inner.clone(), context),
        )
    }

    /// Creates a new node with the given children
    ///
    /// Use this to create container nodes that have child elements.
    /// The children must already exist in the tree.
    ///
    /// @param style - The style configuration for the node
    /// @param children - Array of child node IDs (as BigUint64Array)
    ///
    /// @returns - The node ID (`bigint`)
    ///
    /// @throws `TaffyError` if the node cannot be created
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const containerStyle = new Style();
    /// containerStyle.display = Display.Flex;
    ///
    /// const child1: bigint = tree.newLeaf(new Style());
    /// const child2: bigint = tree.newLeaf(new Style());
    ///
    /// const container: bigint = tree.newWithChildren(
    ///   containerStyle,
    ///   BigUint64Array.from([child1, child2])
    /// );
    /// ```
    #[wasm_bindgen(js_name = newWithChildren)]
    pub fn new_with_children(
        &mut self,
        style: &JsStyle,
        children: Box<[u64]>,
    ) -> Result<u64, JsValue> {
        let children_ids: Vec<NodeId> = children.iter().map(|&id| NodeId::from(id)).collect();
        map_node_result(
            self.tree
                .new_with_children(style.inner.clone(), &children_ids),
        )
    }

    // =========================================================================
    // Tree Operations
    // =========================================================================

    /// Removes all nodes from the tree
    ///
    /// This clears the entire tree, removing all nodes and their relationships.
    /// Use this to reset the tree for reuse.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// tree.clear();
    /// console.log(tree.totalNodeCount());
    /// ```
    #[wasm_bindgen(js_name = clear)]
    pub fn clear(&mut self) {
        self.tree.clear();
    }

    /// Removes a node from the tree
    ///
    /// The node and all its descendants are removed. If the node has a parent,
    /// it is automatically removed from the parent's children.
    ///
    /// @param node - The node ID to remove
    ///
    /// @returns - The removed node ID (`bigint`)
    ///
    /// @throws `TaffyError` if the node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// try {
    ///   const removedId: bigint = tree.remove(nodeId);
    /// } catch (e) {
    ///   console.error("Node doesn't exist");
    /// }
    /// ```
    #[wasm_bindgen(js_name = remove)]
    pub fn remove(&mut self, node: u64) -> Result<u64, JsValue> {
        map_node_result(self.tree.remove(NodeId::from(node)))
    }

    // =========================================================================
    // Node Context
    // =========================================================================

    /// Sets a context value for a node
    ///
    /// The context can be any JavaScript value and is passed to the measure
    /// function during layout computation.
    ///
    /// @param node - The node ID
    /// @param context - Any JavaScript value to attach
    ///
    /// @throws `TaffyError` if the node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// interface Context { text: string };
    /// tree.setNodeContext(nodeId, { text: "Updated text" } as Context);
    /// ```
    #[wasm_bindgen(js_name = setNodeContext)]
    pub fn set_node_context(&mut self, node: u64, context: JsValue) -> Result<(), JsValue> {
        map_void_result(
            self.tree
                .set_node_context(NodeId::from(node), Some(context)),
        )
    }

    /// Gets the context value for a node
    ///
    /// @param node - The node ID
    ///
    /// @returns - The attached context value, or `undefined` if none is set
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// interface Context { text: string };
    /// const context = tree.getNodeContext(nodeId) as Context | undefined;
    /// if (context) {
    ///   console.log(context.text);
    /// }
    /// ```
    #[wasm_bindgen(js_name = getNodeContext)]
    pub fn get_node_context(&self, node: u64) -> Result<JsValue, JsValue> {
        match self.tree.get_node_context(NodeId::from(node)) {
            Some(ctx) => Ok(ctx.clone()),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Gets a mutable reference to the context value for a node
    ///
    /// In JavaScript, this behaves the same as `getNodeContext()` since
    /// JavaScript objects are always passed by reference.
    ///
    /// @param node - The node ID
    ///
    /// @returns - The attached context value, or `undefined` if none is set
    #[wasm_bindgen(js_name = getNodeContextMut)]
    pub fn get_node_context_mut(&mut self, node: u64) -> Result<JsValue, JsValue> {
        match self.tree.get_node_context_mut(NodeId::from(node)) {
            Some(ctx) => Ok(ctx.clone()),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Gets context values for multiple nodes at once
    ///
    /// This is more efficient than calling `getNodeContext()` multiple times
    /// when you need to access contexts for many nodes.
    ///
    /// @param children - Array of node IDs
    ///
    /// @returns - Array of context values (undefined for nodes without context)
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const id1 = tree.newLeaf(new Style());
    /// const id2 = tree.newLeaf(new Style());
    /// const nodes = BigUint64Array.from([id1, id2]);
    /// const contexts = tree.getDisjointNodeContextMut(nodes);
    /// ```
    #[wasm_bindgen(js_name = getDisjointNodeContextMut)]
    pub fn get_disjoint_node_context_mut(
        &mut self,
        children: Box<[u64]>,
    ) -> Result<Box<[JsValue]>, JsValue> {
        let mut results = Vec::with_capacity(children.len());
        for id in children.iter() {
            match self.tree.get_node_context_mut(NodeId::from(*id)) {
                Some(ctx) => results.push(ctx.clone()),
                None => results.push(JsValue::UNDEFINED),
            }
        }
        Ok(results.into_boxed_slice())
    }

    // =========================================================================
    // Child Management
    // =========================================================================

    /// Appends a child node to a parent
    ///
    /// The child is added as the last child of the parent.
    ///
    /// @param parent - The parent node ID
    /// @param child - The child node ID to add
    ///
    /// @throws `TaffyError` if the parent or child node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// ```
    #[wasm_bindgen(js_name = addChild)]
    pub fn add_child(&mut self, parent: u64, child: u64) -> Result<(), JsValue> {
        map_void_result(
            self.tree
                .add_child(NodeId::from(parent), NodeId::from(child)),
        )
    }

    /// Inserts a child at a specific index
    ///
    /// @param parent - The parent node ID
    /// @param index - The position to insert at (0-based)
    /// @param child - The child node ID to insert
    ///
    /// @throws `TaffyError` if the parent or child node does not exist, or index is out of bounds
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.insertChildAtIndex(parentId, 0, childId);
    /// ```
    #[wasm_bindgen(js_name = insertChildAtIndex)]
    pub fn insert_child_at_index(
        &mut self,
        parent: u64,
        index: usize,
        child: u64,
    ) -> Result<(), JsValue> {
        map_void_result(self.tree.insert_child_at_index(
            NodeId::from(parent),
            index,
            NodeId::from(child),
        ))
    }

    /// Replaces all children of a node
    ///
    /// Any existing children are removed and replaced with the new array.
    ///
    /// @param parent - The parent node ID
    /// @param children - Array of new child node IDs
    ///
    /// @throws `TaffyError` if the parent node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const child1 = tree.newLeaf(new Style());
    /// const child2 = tree.newLeaf(new Style());
    /// const child3 = tree.newLeaf(new Style());
    /// const children = BigUint64Array.from([child1, child2, child3]);
    /// tree.setChildren(parentId, children);
    /// ```
    #[wasm_bindgen(js_name = setChildren)]
    pub fn set_children(&mut self, parent: u64, children: Box<[u64]>) -> Result<(), JsValue> {
        let children_ids: Vec<NodeId> = children.iter().map(|&id| NodeId::from(id)).collect();
        map_void_result(self.tree.set_children(NodeId::from(parent), &children_ids))
    }

    /// Removes a specific child from a parent
    ///
    /// @param parent - The parent node ID
    /// @param child - The child node ID to remove
    ///
    /// @returns - The removed child ID (`bigint`)
    ///
    /// @throws `TaffyError` if the parent or child node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// tree.removeChild(parentId, childId);
    /// ```
    #[wasm_bindgen(js_name = removeChild)]
    pub fn remove_child(&mut self, parent: u64, child: u64) -> Result<u64, JsValue> {
        map_node_result(
            self.tree
                .remove_child(NodeId::from(parent), NodeId::from(child)),
        )
    }

    /// Removes a child at a specific index
    ///
    /// @param parent - The parent node ID
    /// @param index - The index of the child to remove (0-based)
    ///
    /// @returns - The removed child ID (`bigint`)
    ///
    /// @throws `TaffyError` if the parent node does not exist or index is out of bounds
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// const removedId: bigint = tree.removeChildAtIndex(parentId, 0);
    /// ```
    #[wasm_bindgen(js_name = removeChildAtIndex)]
    pub fn remove_child_at_index(&mut self, parent: u64, index: usize) -> Result<u64, JsValue> {
        map_node_result(self.tree.remove_child_at_index(NodeId::from(parent), index))
    }

    /// Replaces a child at a specific index
    ///
    /// @param parent - The parent node ID
    /// @param index - The index of the child to replace (0-based)
    /// @param newChild - The new child node ID
    ///
    /// @returns - The replaced (old) child ID (`bigint`)
    ///
    /// @throws `TaffyError` if the parent node does not exist or index is out of bounds
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const oldChild = tree.newLeaf(new Style());
    /// const newChildId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, oldChild);
    /// const child = tree.newLeaf(new Style()); // filler child at index 0 if needed, but index 1 implies 2 children
    /// tree.insertChildAtIndex(parentId, 0, child);
    ///
    /// const oldChildId: bigint = tree.replaceChildAtIndex(parentId, 1, newChildId);
    /// ```
    #[wasm_bindgen(js_name = replaceChildAtIndex)]
    pub fn replace_child_at_index(
        &mut self,
        parent: u64,
        index: usize,
        #[wasm_bindgen(js_name = "newChild")] new_child: u64,
    ) -> Result<u64, JsValue> {
        map_node_result(self.tree.replace_child_at_index(
            NodeId::from(parent),
            index,
            NodeId::from(new_child),
        ))
    }

    /// Gets the child at a specific index
    ///
    /// @param parent - The parent node ID
    /// @param index - The index of the child (0-based)
    ///
    /// @returns - The child node ID (`bigint`)
    ///
    /// @throws `TaffyError` if the parent node does not exist or index is out of bounds
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// const firstChild: bigint = tree.getChildAtIndex(parentId, 0);
    /// ```
    #[wasm_bindgen(js_name = getChildAtIndex)]
    pub fn get_child_at_index(&self, parent: u64, index: usize) -> Result<u64, JsValue> {
        map_node_result(self.tree.child_at_index(NodeId::from(parent), index))
    }

    /// Removes a range of children
    ///
    /// Removes children from `start_index` (inclusive) to `end_index` (exclusive).
    ///
    /// @param parent - The parent node ID
    /// @param startIndex - Start of range (inclusive)
    /// @param endIndex - End of range (exclusive)
    ///
    /// @throws `TaffyError` if the parent node does not exist or range is invalid
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const child1 = tree.newLeaf(new Style());
    /// const child2 = tree.newLeaf(new Style());
    /// const child3 = tree.newLeaf(new Style());
    /// tree.setChildren(parentId, BigUint64Array.from([child1, child2, child3]));
    ///
    /// tree.removeChildrenRange(parentId, 1, 3);
    /// ```
    #[wasm_bindgen(js_name = removeChildrenRange)]
    pub fn remove_children_range(
        &mut self,
        parent: u64,
        #[wasm_bindgen(js_name = "startIndex")] start_index: usize,
        #[wasm_bindgen(js_name = "endIndex")] end_index: usize,
    ) -> Result<(), JsValue> {
        map_void_result(
            self.tree
                .remove_children_range(NodeId::from(parent), start_index..end_index),
        )
    }

    /// Gets the total number of nodes in the tree
    ///
    /// @returns - The total count of all nodes
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const count: number = tree.totalNodeCount();
    /// ```
    #[wasm_bindgen(js_name = totalNodeCount)]
    pub fn total_node_count(&self) -> usize {
        self.tree.total_node_count()
    }

    /// Gets the number of children of a node
    ///
    /// @param parent - The parent node ID
    ///
    /// @returns - The number of direct children
    ///
    /// @throws `TaffyError` if the node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const count: number = tree.childCount(parentId);
    /// ```
    #[wasm_bindgen(js_name = childCount)]
    pub fn child_count(&self, parent: u64) -> usize {
        self.tree.child_count(NodeId::from(parent))
    }

    /// Gets the parent of a node
    ///
    /// @param child - The child node ID
    ///
    /// @returns - The parent node ID, or `undefined` if the node has no parent
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// const parent: bigint | undefined = tree.parent(childId);
    /// ```
    #[wasm_bindgen(js_name = parent)]
    pub fn parent(&self, child: u64) -> Option<u64> {
        self.tree.parent(NodeId::from(child)).map(u64::from)
    }

    /// Gets all children of a node
    ///
    /// @param parent - The parent node ID
    ///
    /// @returns - Array of child node IDs (`BigUint64Array`)
    ///
    /// @throws `TaffyError` if the parent node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const children: BigUint64Array = tree.children(parentId);
    /// ```
    #[wasm_bindgen(js_name = children)]
    pub fn children(&self, parent: u64) -> Result<Box<[u64]>, JsValue> {
        self.tree
            .children(NodeId::from(parent))
            .map(|c| c.into_iter().map(u64::from).collect::<Box<[u64]>>())
            .map_err(to_js_error)
    }

    // =========================================================================
    // Style Management
    // =========================================================================

    /// Sets the style for an existing node
    ///
    /// This replaces the node's current style with the provided one.
    /// The node will be marked as dirty and require re-layout.
    ///
    /// @param node - The node ID
    /// @param style - The new style configuration
    ///
    /// @throws `TaffyError` if the node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// const newStyle = new Style();
    /// newStyle.flexGrow = 2;
    /// tree.setStyle(nodeId, newStyle);
    /// ```
    #[wasm_bindgen(js_name = setStyle)]
    pub fn set_style(&mut self, node: u64, style: &JsStyle) -> Result<(), JsValue> {
        map_void_result(self.tree.set_style(NodeId::from(node), style.inner.clone()))
    }

    /// Gets the style for a node
    ///
    /// @param node - The node ID
    ///
    /// @returns - The node's `Style`
    ///
    /// @throws `TaffyError` if the node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// const style: Style = tree.getStyle(nodeId);
    /// console.log('Flex grow:', style.flexGrow);
    /// ```
    #[wasm_bindgen(js_name = getStyle)]
    pub fn style(&self, node: u64) -> Result<JsStyle, JsValue> {
        match self.tree.style(NodeId::from(node)) {
            Ok(s) => Ok(JsStyle { inner: s.clone() }),
            Err(e) => Err(JsValue::from(JsTaffyError::from(e))),
        }
    }

    // =========================================================================
    // Layout Results
    // =========================================================================

    /// Gets the computed layout for a node
    ///
    /// Call this after `computeLayout()` to retrieve the computed position
    /// and size for a node.
    ///
    /// @param node - The node ID
    ///
    /// @returns - The computed `Layout`
    ///
    /// @throws `TaffyError` if the node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const style = new Style();
    /// style.size = { width: 100, height: 100 };
    /// const rootId = tree.newLeaf(style);
    /// const nodeId = rootId;
    ///
    /// tree.computeLayout(rootId, { width: 800, height: 600 });
    /// const layout: Layout = tree.getLayout(nodeId);
    /// console.log(`Position: (${layout.x}, ${layout.y}), Size: ${layout.width}x${layout.height}`);
    /// ```
    #[wasm_bindgen(js_name = getLayout)]
    pub fn layout(&self, node: u64) -> Result<JsLayout, JsValue> {
        match self.tree.layout(NodeId::from(node)) {
            Ok(l) => Ok(JsLayout::from(l)),
            Err(e) => Err(JsValue::from(JsTaffyError::from(e))),
        }
    }

    /// Gets the unrounded (fractional) layout for a node
    ///
    /// Returns the raw computed values before any rounding is applied.
    /// Useful when you need sub-pixel precision.
    ///
    /// @param node - The node ID
    ///
    /// @returns - The unrounded `Layout`
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// const layout: Layout = tree.unroundedLayout(nodeId);
    /// console.log(`Exact width: ${layout.width}`);
    /// ```
    #[wasm_bindgen(js_name = unroundedLayout)]
    pub fn unrounded_layout(&self, node: u64) -> JsLayout {
        JsLayout::from(self.tree.unrounded_layout(NodeId::from(node)))
    }

    /// Gets detailed layout information for grid layouts
    ///
    /// @note
    /// This method is only available when the `detailed_layout_info`
    /// feature is enabled.
    ///
    /// @param node - The node ID
    ///
    /// @returns - Detailed grid info or "None" for non-grid nodes
    ///
    /// @throws `TaffyError` if the node does not exist
    #[cfg(feature = "detailed_layout_info")]
    #[wasm_bindgen(js_name = detailedLayoutInfo)]
    pub fn detailed_layout_info(&self, node: u64) -> Result<JsValue, JsValue> {
        match self.tree.detailed_layout_info(NodeId::from(node)) {
            DetailedLayoutInfo::Grid(info) => {
                let dto = DetailedGridInfoDto {
                    rows: DetailedGridTracksInfoDto {
                        negative_implicit_tracks: info.rows.negative_implicit_tracks,
                        explicit_tracks: info.rows.explicit_tracks,
                        positive_implicit_tracks: info.rows.positive_implicit_tracks,
                        gutters: info.rows.gutters.clone(),
                        sizes: info.rows.sizes.clone(),
                    },
                    columns: DetailedGridTracksInfoDto {
                        negative_implicit_tracks: info.columns.negative_implicit_tracks,
                        explicit_tracks: info.columns.explicit_tracks,
                        positive_implicit_tracks: info.columns.positive_implicit_tracks,
                        gutters: info.columns.gutters.clone(),
                        sizes: info.columns.sizes.clone(),
                    },
                    items: info
                        .items
                        .iter()
                        .map(|item| DetailedGridItemsInfoDto {
                            row_start: item.row_start,
                            row_end: item.row_end,
                            column_start: item.column_start,
                            column_end: item.column_end,
                        })
                        .collect(),
                };
                Ok(serde_wasm_bindgen::to_value(&dto).unwrap_or(JsValue::NULL))
            }
            DetailedLayoutInfo::None => Ok(JsValue::NULL),
        }
    }

    // =========================================================================
    // Dirty Tracking
    // =========================================================================

    /// Marks a node as dirty (requiring re-layout)
    ///
    /// Use this when a node's content has changed but its style hasn't.
    /// For example, when text content changes and needs remeasuring.
    ///
    /// @param node - The node ID to mark dirty
    ///
    /// @throws `TaffyError` if the node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    /// const nodeId = rootId;
    /// const availableSpace = { width: 100, height: 100 };
    ///
    /// // After updating text content
    /// tree.setNodeContext(nodeId, { text: "Updated text" });
    /// tree.markDirty(nodeId);
    /// tree.computeLayout(rootId, availableSpace);
    /// ```
    #[wasm_bindgen(js_name = markDirty)]
    pub fn mark_dirty(&mut self, node: u64) -> Result<(), JsValue> {
        map_void_result(self.tree.mark_dirty(NodeId::from(node)))
    }

    /// Checks if a node is dirty (needs re-layout)
    ///
    /// A node is dirty if its style or content has changed since the last
    /// layout computation.
    ///
    /// @param node - The node ID to check
    ///
    /// @returns - true if dirty, false otherwise
    ///
    /// @throws `TaffyError` if the node does not exist
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    /// const nodeId = rootId;
    /// const availableSpace = { width: 100, height: 100 };
    ///
    /// if (tree.dirty(nodeId)) {
    ///   tree.computeLayout(rootId, availableSpace);
    /// }
    /// ```
    #[wasm_bindgen(js_name = dirty)]
    pub fn dirty(&self, node: u64) -> Result<bool, JsValue> {
        map_bool_result(self.tree.dirty(NodeId::from(node)))
    }

    // =========================================================================
    // Layout Computation
    // =========================================================================

    /// Computes layout with a custom measure function for leaf nodes
    ///
    /// Use this when you have leaf nodes with dynamic content (like text)
    /// that needs to be measured during layout. The measure function is
    /// called for each leaf node that needs measurement.
    ///
    /// @param node - The root node ID to compute layout for
    /// @param availableSpace - The available space constraints
    /// @param measureFunc - A function that measures leaf node content
    ///
    /// @throws `TaffyError` if the node does not exist or available space is invalid
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    ///
    /// const measureText = (text: string, width: number) => ({ width: 0, height: 0 });
    ///
    /// tree.computeLayoutWithMeasure(
    ///   rootId,
    ///   { width: 800, height: "max-content" },
    ///   (known, available, node, context, style) => {
    ///     if (context?.text) {
    ///       const measured = measureText(context.text, available.width as number);
    ///       return { width: measured.width, height: measured.height };
    ///     }
    ///     return { width: 0, height: 0 };
    ///   }
    /// );
    /// ```
    #[wasm_bindgen(js_name = computeLayoutWithMeasure)]
    pub fn compute_layout_with_measure(
        &mut self,
        node: u64,
        #[wasm_bindgen(js_name = "availableSpace")] available_space: JsAvailableSizeArg,
        #[wasm_bindgen(js_name = "measureFunc")] measure_func: JsMeasureFunctionArg,
    ) -> Result<(), JsValue> {
        let js_value: JsValue = available_space.unchecked_into();
        let js_space = match serde_wasm_bindgen::from_value::<AvailableSizeDto>(js_value) {
            Ok(s) => s,
            Err(_) => {
                return Err(JsValue::from(JsTaffyError::from(
                    NativeTaffyError::InvalidInputNode(NodeId::from(node)),
                )));
            }
        };

        let space: Size<AvailableSpace> = js_space.into();
        let func: js_sys::Function = measure_func.unchecked_into();
        let measure = |known_dimensions: Size<Option<f32>>,
                       available_space: Size<AvailableSpace>,
                       _node: NodeId,
                       context: Option<&mut JsValue>,
                       _style: &TaffyStyle::Style|
         -> Size<f32> {
            let this = JsValue::NULL;
            let known_val =
                serde_wasm_bindgen::to_value(&known_dimensions).unwrap_or(JsValue::NULL);
            let available_dto = AvailableSizeDto {
                width: available_space.width.into(),
                height: available_space.height.into(),
            };
            let available_val =
                serde_wasm_bindgen::to_value(&available_dto).unwrap_or(JsValue::NULL);
            let ctx = context.cloned().unwrap_or(JsValue::UNDEFINED);
            let style = JsStyle {
                inner: _style.clone(),
            };
            let style_val = JsValue::from(style);
            let node_id: u64 = _node.into();
            let node_val = JsValue::from(node_id);
            let args = js_sys::Array::new();
            args.push(&known_val);
            args.push(&available_val);
            args.push(&node_val);
            args.push(&ctx);
            args.push(&style_val);
            let result_val = func.apply(&this, &args).unwrap_or(JsValue::UNDEFINED);
            serde_wasm_bindgen::from_value(result_val).unwrap_or(Size::ZERO)
        };
        map_void_result(
            self.tree
                .compute_layout_with_measure(NodeId::from(node), space, measure),
        )
    }

    /// Computes the layout for a subtree
    ///
    /// This is the main layout computation method. Call this on the root node
    /// to compute layouts for all nodes in the tree.
    ///
    /// @param node - The root node ID to compute layout for
    /// @param availableSpace - The available space constraints
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    ///
    /// // Fixed size container
    /// tree.computeLayout(rootId, { width: 800, height: 600 });
    ///
    /// // Flexible width, fixed height
    /// tree.computeLayout(rootId, { width: "max-content", height: 600 });
    ///
    /// // Minimum content size
    /// tree.computeLayout(rootId, { width: "min-content", height: "min-content" });
    /// ```
    ///
    /// @throws `TaffyError` if the node does not exist or available space is invalid
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    /// tree.computeLayout(rootId, { width: 800, height: 600 });
    /// ```
    #[wasm_bindgen(js_name = computeLayout)]
    pub fn compute_layout(
        &mut self,
        node: u64,
        #[wasm_bindgen(js_name = "availableSpace")] available_space: JsAvailableSizeArg,
    ) -> Result<(), JsValue> {
        let js_value: JsValue = available_space.unchecked_into();
        match serde_wasm_bindgen::from_value::<AvailableSizeDto>(js_value) {
            Ok(js_space) => {
                let space: Size<AvailableSpace> = js_space.into();
                map_void_result(self.tree.compute_layout(NodeId::from(node), space))
            }
            Err(_) => Err(JsValue::from(JsTaffyError::from(
                NativeTaffyError::InvalidInputNode(NodeId::from(node)),
            ))),
        }
    }

    // =========================================================================
    // Utilities
    // =========================================================================

    /// Prints the tree structure to the console (for debugging)
    ///
    /// Outputs a text representation of the tree structure starting from
    /// the given node. Useful for debugging layout issues.
    ///
    /// @param node - The root node ID to print from
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    /// tree.printTree(rootId);
    /// // Output appears in browser console
    /// ```
    #[wasm_bindgen(js_name = printTree)]
    pub fn print_tree(&mut self, node: u64) {
        self.tree.print_tree(NodeId::from(node));
    }
}
