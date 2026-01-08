//! # Error Handling Module
//!
//! This module provides error handling types and utilities for the Taffy Layout bindings.
//! It wraps the native Taffy error types and provides helper functions for mapping
//! Rust `Result` types to JavaScript exceptions.
//!
//! ## Error Type
//!
//! The [`JsTaffyError`] struct wraps a native `taffy::TaffyError` and exposes a `message`
//! property to JavaScript for error introspection.
//!
//! ## Result Mapping
//!
//! Helper functions are provided for converting Rust `Result` types to throw exceptions:
//!
//! - [`map_node_result`]: For results containing `NodeId` values
//! - [`map_void_result`]: For results with no return value
//! - [`map_bool_result`]: For results containing boolean values
//!
//! try {
//!   const tree = new TaffyTree();
//!   const style = new Style();
//!   const nodeId = tree.newLeaf(style);
//!   console.log('Created node:', nodeId);
//! } catch (e) {
//!   // e is a TaffyError instance
//!   if (e instanceof TaffyError) {
//!     console.error('Layout error:', e.message);
//!   }
//! }
//! ```

use taffy::TaffyError;
use taffy::prelude::NodeId;
use wasm_bindgen::prelude::*;

// =============================================================================
// Error Wrapper
// =============================================================================

/// Error class thrown when a Taffy operation fails, containing a human-readable error message.
///
/// This class wraps the native [`taffy::TaffyError`] type and exposes it to JavaScript
/// with a readable error message. It is thrown as a JavaScript exception on failure.
///
/// @example
/// ```typescript
/// try {
///  const tree = new TaffyTree();
///  const node = tree.newLeaf(new Style());
///  tree.remove(node);
/// } catch (e) {
///   if (e instanceof TaffyError) {
///      console.error(e.message);
///   }
/// }
/// ```
///
/// @remarks
/// The underlying Taffy errors include:
/// - `InvalidInputNode`: Node ID doesn't exist in the tree
/// - `InvalidParentNode`: Specified parent node doesn't exist
/// - `ChildIndexOutOfBounds`: Child index exceeds available children
#[wasm_bindgen(js_name = TaffyError)]
pub struct JsTaffyError {
    /// The wrapped native Taffy error
    inner: TaffyError,
}

#[wasm_bindgen(js_class = "TaffyError")]
impl JsTaffyError {
    /// Gets the human-readable error message
    ///
    /// @returns - A string describing what went wrong.
    ///
    /// @remarks
    /// Examples:
    /// - "Node with id 1234 is not present in the Taffy tree"
    /// - "Index 5 is out of bounds for node with 3 children"
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.inner.to_string()
    }
}

impl From<TaffyError> for JsTaffyError {
    fn from(inner: TaffyError) -> Self {
        JsTaffyError { inner }
    }
}

// =============================================================================
// Result Mapping Utilities
// =============================================================================

/// Creates a JsValue error from a TaffyError
///
/// This function converts a `TaffyError` to a `JsValue` that can be thrown
/// as a JavaScript exception.
///
/// @param e - The Taffy error to convert
///
/// @returns - A `JsValue` containing the error message
pub(crate) fn to_js_error(e: TaffyError) -> JsValue {
    JsValue::from(JsTaffyError::from(e))
}

/// Maps a NodeId Result to a JavaScript bigint
///
/// Specialized version that converts the `NodeId` to a `u64` (BigInt in JavaScript).
/// Throws a JavaScript exception on error.
///
/// @param result - A result containing a `NodeId` on success
///
/// @returns - A `u64` node ID on success, or throws `TaffyError` on failure
///
/// @example
/// ```typescript
/// const tree = new TaffyTree();
/// const style = new Style();
/// const nodeId = tree.newLeaf(style);  // Returns bigint or throws TaffyError
/// ```
pub(crate) fn map_node_result(result: Result<NodeId, TaffyError>) -> Result<u64, JsValue> {
    match result {
        Ok(id) => Ok(u64::from(id)),
        Err(e) => Err(to_js_error(e)),
    }
}

/// Maps a void Result to a JavaScript Result
///
/// Specialized version for `Result<(), TaffyError>`.
/// Returns nothing on success, or throws a JavaScript exception on failure.
///
/// @param result - A result with no success value
///
/// @returns - Nothing on success, or throws `TaffyError` on failure
///
/// @example
/// ```typescript
/// const tree = new TaffyTree();
/// const nodeId = tree.newLeaf(new Style());
/// const style = new Style();
/// tree.setStyle(nodeId, style);  // Returns void or throws TaffyError
/// ```
pub(crate) fn map_void_result(result: Result<(), TaffyError>) -> Result<(), JsValue> {
    result.map_err(to_js_error)
}

/// Maps a bool Result to a JavaScript boolean
///
/// Specialized version for `Result<bool, TaffyError>`.
/// Returns the boolean on success, or throws a JavaScript exception on failure.
///
/// @param result - A result containing a boolean on success
///
/// @returns - A boolean on success, or throws `TaffyError` on failure
pub(crate) fn map_bool_result(result: Result<bool, TaffyError>) -> Result<bool, JsValue> {
    result.map_err(to_js_error)
}
