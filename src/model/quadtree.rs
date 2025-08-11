use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadTreeBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl QuadTreeBounds {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center_x(&self) -> f64 {
        self.x + self.width / 2.0
    }

    pub fn center_y(&self) -> f64 {
        self.y + self.height / 2.0
    }

    // Check if this bounds fully contains a circle
    pub fn contains_circle(&self, cx: f64, cy: f64, radius: f64) -> bool {
        let left = self.x;
        let right = self.x + self.width;
        let bottom = self.y;
        let top = self.y + self.height;

        // Circle is fully contained if all extremes are within bounds
        cx - radius >= left && cx + radius <= right && cy - radius >= bottom && cy + radius <= top
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadTreeNode {
    pub bounds: QuadTreeBounds,
    pub has_obstacle: bool,
    pub children: Option<Box<[QuadTreeNode; 4]>>,
    pub is_leaf: bool,
}

impl QuadTreeNode {
    /// Create a new quad-tree node
    pub fn new(bounds: QuadTreeBounds) -> Self {
        Self {
            bounds,
            has_obstacle: false,
            children: None,
            is_leaf: true,
        }
    }

    /// Split this node into four children
    /// This will create four quadrants: NW, NE, SW, SE
    fn split(&mut self) {
        let half_width = self.bounds.width / 2.0;
        let half_height = self.bounds.height / 2.0;

        let cx = self.bounds.center_x();
        let cy = self.bounds.center_y();

        // Create four quadrants: NW, NE, SW, SE
        let nw = QuadTreeNode::new(QuadTreeBounds::new(
            self.bounds.x,
            cy,
            half_width,
            half_height,
        ));
        let ne = QuadTreeNode::new(QuadTreeBounds::new(cx, cy, half_width, half_height));
        let sw = QuadTreeNode::new(QuadTreeBounds::new(
            self.bounds.x,
            self.bounds.y,
            half_width,
            half_height,
        ));
        let se = QuadTreeNode::new(QuadTreeBounds::new(
            cx,
            self.bounds.y,
            half_width,
            half_height,
        ));

        self.children = Some(Box::new([sw, se, nw, ne]));
        self.is_leaf = false;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadTree {
    pub root: QuadTreeNode,
    pub min_node_size: f64, // Minimum size before stopping subdivision
}

impl QuadTree {
    /// Create a new quad-tree with the given bounds and minimum node size
    pub fn new(bounds: QuadTreeBounds, min_node_size: f64) -> Self {
        Self {
            root: QuadTreeNode::new(bounds),
            min_node_size,
        }
    }

    /// Build the quad-tree from a grid, taking into account the cutter radius for margins
    pub fn build_from_grid(
        grid: &super::grid::Grid,
        cutter_radius: f64,
        min_qnode_size: f64,
    ) -> Self {
        let bounds = QuadTreeBounds {
            x: 0.0,
            y: 0.0,
            width: grid.cells_x as f64 * grid.cell_size,
            height: grid.cells_y as f64 * grid.cell_size,
        };

        let min_node_size = cutter_radius * min_qnode_size;
        let mut tree = Self::new(bounds, min_node_size);

        // Build the tree recursively
        {
            let root = &mut tree.root;
            QuadTree::build_node(root, grid, cutter_radius, min_node_size);
        }
        tree
    }

    fn build_node(
        node: &mut QuadTreeNode,
        grid: &super::grid::Grid,
        cutter_radius: f64,
        min_node_size: f64,
    ) {
        // Convert bounds to grid coordinates
        let start_x = (node.bounds.x / grid.cell_size).floor() as usize;
        let start_y = (node.bounds.y / grid.cell_size).floor() as usize;
        let end_x = ((node.bounds.x + node.bounds.width) / grid.cell_size).ceil() as usize;
        let end_y = ((node.bounds.y + node.bounds.height) / grid.cell_size).ceil() as usize;

        // Check for obstacles in this node, including margin for cutter radius
        let margin = (cutter_radius / grid.cell_size).ceil() as usize;
        let mut has_obstacle = false;

        // Expanded search area to include margin
        for y in start_y.saturating_sub(margin)..end_y.saturating_add(margin).min(grid.cells_y) {
            for x in start_x.saturating_sub(margin)..end_x.saturating_add(margin).min(grid.cells_x)
            {
                if let Some(cell) = grid.get_cell(x, y)
                    && cell.is_obstacle()
                {
                    has_obstacle = true;
                    break;
                }
            }
            if has_obstacle {
                break;
            }
        }

        node.has_obstacle = has_obstacle;

        // If node has obstacles and is larger than minimum size, subdivide
        if has_obstacle && node.bounds.width > min_node_size {
            node.split();
            if let Some(children) = &mut node.children {
                for child in children.iter_mut() {
                    QuadTree::build_node(child, grid, cutter_radius, min_node_size);
                }
            }
        }
    }

    /// Save the quad-tree to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let serialized = serde_json::to_string(self)?;
        fs::write(path, serialized)
    }

    #[allow(dead_code)]
    /// Load a quad-tree from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let tree: QuadTree = serde_json::from_str(&contents)?;
        Ok(tree)
    }

    /// Find the smallest leaf node that fully contains the given circle
    /// Returns None if no node can fully contain the circle
    pub fn find_node_containing_circle(
        node: &QuadTreeNode,
        cx: f64,
        cy: f64,
        radius: f64,
    ) -> Option<&QuadTreeNode> {
        // First check if this node can contain the circle at all
        if !node.bounds.contains_circle(cx, cy, radius) {
            return None;
        }

        // If this is a leaf node and it contains the circle, return it
        if node.is_leaf {
            return Some(node);
        }

        // If not a leaf, check children
        if let Some(ref children) = node.children {
            // Try to find a child that can fully contain the circle
            for child in children.iter() {
                if let Some(containing_leaf) =
                    QuadTree::find_node_containing_circle(child, cx, cy, radius)
                {
                    return Some(containing_leaf);
                }
            }
        }

        // If no child can fully contain the circle, this node is the answer
        // This can happen when the circle spans multiple child quadrants
        Some(node)
    }

    /// Check if a position might collide with obstacles
    /// This is done by finding the smallest leaf node that fully contains the circle
    /// If that leaf has an obstacle, we return true as we need to do a detailed collision check
    /// If thatb leaf node does not have an obstacle, we return false as there is no need for a detailed collisoin check
    pub fn might_have_collision(&self, cx: f64, cy: f64, radius: f64) -> bool {
        // self.leaf_has_obstacle(&self.root, cx, cy, radius)
        if let Some(node) = QuadTree::find_node_containing_circle(&self.root, cx, cy, radius) {
            // If we found a containing leaf, check if it has an obstacle
            return node.has_obstacle;
        }
        true
    }
}
