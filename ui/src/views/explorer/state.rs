const DEFAULT_TREE_WIDTH_PX: f64 = 320.0;
const MIN_TREE_WIDTH_PX: f64 = 180.0;

/// Page-local layout state: the width of the tree pane and whether the splitter is being dragged.
/// Kept apart from `ExplorerState` so dragging re-renders the page shell only, never the panes.
pub struct SplitterState {
    pub tree_width: f64,
    pub dragging: bool,
}

impl Default for SplitterState {
    fn default() -> Self {
        Self {
            tree_width: DEFAULT_TREE_WIDTH_PX,
            dragging: false,
        }
    }
}

impl SplitterState {
    pub fn start_drag(&mut self) {
        self.dragging = true;
    }

    pub fn stop_drag(&mut self) {
        self.dragging = false;
    }

    /// The tree pane starts at the left edge of the viewport, so its width is the pointer's
    /// client x. The upper bound is CSS (`max-width` of `.tree-pane`), which knows the viewport.
    pub fn drag_to(&mut self, client_x: f64) {
        self.tree_width = client_x.round().max(MIN_TREE_WIDTH_PX);
    }
}
