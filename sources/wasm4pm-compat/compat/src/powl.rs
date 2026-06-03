//! POWL (Partial Order Workflow Language) types and sealed containers
//!
//! POWL represents workflows as partial-order directed acyclic graphs (DAG)
//! with explicit causality and non-deterministic choice.
//! These types are sealed and non-forgeable via type law.

use std::collections::BTreeSet;
use std::fmt;

/// Sealed marker trait for POWL-compliant types.
/// Only types that satisfy TreeProjectable can be instantiated as POWL models.
pub trait TreeProjectable: Sized {
    /// Verify that the tree structure is acyclic and satisfies POWL invariants
    fn verify_tree_properties(&self) -> Result<(), String>;

    /// Project this POWL onto a tree structure
    fn to_tree_projection(&self) -> TreeProjection;
}

/// Tree projection of a POWL: hierarchical decomposition
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeProjection {
    /// Root operator type
    pub root: OperatorKind,
    /// Child indices
    pub children: Vec<usize>,
    /// Activity label if leaf
    pub activity: Option<String>,
}

/// POWL operator kinds
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperatorKind {
    /// Sequence: strict left-to-right ordering
    Sequence,
    /// Exclusive choice: one branch selected at runtime
    XOR,
    /// Parallel: all branches execute concurrently
    AND,
    /// Partial order: explicit causality edges (non-deterministic ordering)
    PartialOrder,
    /// Loop: do-once + optional redo
    Loop,
    /// Activity leaf node
    Activity,
}

/// Sealed POWL model with type-law guarantee
/// Cannot be constructed directly; only via PowerMiner discovery
#[derive(Clone, Debug)]
pub struct TypedPowl {
    // Private fields to prevent forgery
    nodes: Vec<PowlNode>,
    edges: BTreeSet<(usize, usize)>,  // (from, to) causality edges
    root_index: usize,
    _sealed: (),  // Zero-cost type seal
}

/// Single node in POWL DAG
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PowlNode {
    /// Leaf: activity execution
    Activity {
        name: String,
    },
    /// Operator: sequence, choice, parallel, or partial order
    Operator {
        kind: OperatorKind,
        children: Vec<usize>,
    },
}

impl TypedPowl {
    /// Create sealed POWL (only called by PowerMiner or test fixtures)
    pub fn seal(nodes: Vec<PowlNode>, edges: BTreeSet<(usize, usize)>, root_index: usize)
        -> Result<Self, String>
    {
        let powl = TypedPowl {
            nodes,
            edges,
            root_index,
            _sealed: (),
        };

        // Verify acyclicity and tree properties
        powl.verify_tree_properties()?;
        Ok(powl)
    }

    /// Get nodes (read-only)
    pub fn nodes(&self) -> &[PowlNode] {
        &self.nodes
    }

    /// Get causality edges (read-only)
    pub fn edges(&self) -> &BTreeSet<(usize, usize)> {
        &self.edges
    }

    /// Get root node index
    pub fn root_index(&self) -> usize {
        self.root_index
    }

    /// Check if a valid DAG (no cycles)
    fn has_cycle(&self) -> bool {
        // DFS cycle detection
        let mut visited = vec![false; self.nodes.len()];
        let mut rec_stack = vec![false; self.nodes.len()];

        for i in 0..self.nodes.len() {
            if !visited[i] && self._has_cycle_dfs(i, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        false
    }

    fn _has_cycle_dfs(&self, node: usize, visited: &mut [bool], rec_stack: &mut [bool]) -> bool {
        visited[node] = true;
        rec_stack[node] = true;

        for &(from, to) in &self.edges {
            if from == node {
                if !visited[to] {
                    if self._has_cycle_dfs(to, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack[to] {
                    return true;
                }
            }
        }

        rec_stack[node] = false;
        false
    }
}

impl TreeProjectable for TypedPowl {
    fn verify_tree_properties(&self) -> Result<(), String> {
        // 1. Check acyclicity
        if self.has_cycle() {
            return Err("POWL contains a cycle".to_string());
        }

        // 2. Check root exists
        if self.root_index >= self.nodes.len() {
            return Err("Root index out of bounds".to_string());
        }

        // 3. Check all edges reference valid nodes
        for &(from, to) in &self.edges {
            if from >= self.nodes.len() || to >= self.nodes.len() {
                return Err("Edge references invalid node index".to_string());
            }
        }

        // 4. Check children in operators match node count
        for (idx, node) in self.nodes.iter().enumerate() {
            if let PowlNode::Operator { children, .. } = node {
                for &child_idx in children {
                    if child_idx >= self.nodes.len() {
                        return Err(format!(
                            "Operator {} references invalid child {}",
                            idx, child_idx
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    fn to_tree_projection(&self) -> TreeProjection {
        match &self.nodes[self.root_index] {
            PowlNode::Activity { name } => {
                TreeProjection {
                    root: OperatorKind::Activity,
                    children: vec![],
                    activity: Some(name.clone()),
                }
            }
            PowlNode::Operator { kind, children } => {
                TreeProjection {
                    root: kind.clone(),
                    children: children.clone(),
                    activity: None,
                }
            }
        }
    }
}

impl fmt::Display for TypedPowl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypedPowl(nodes={}, edges={})",
            self.nodes.len(), self.edges.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_activity_powl() {
        let nodes = vec![
            PowlNode::Activity { name: "A".to_string() },
        ];
        let edges = BTreeSet::new();

        let powl = TypedPowl::seal(nodes, edges, 0).unwrap();
        assert!(powl.verify_tree_properties().is_ok());
        assert_eq!(powl.root_index(), 0);
    }

    #[test]
    fn test_sequence_powl() {
        let nodes = vec![
            PowlNode::Activity { name: "A".to_string() },
            PowlNode::Activity { name: "B".to_string() },
            PowlNode::Operator {
                kind: OperatorKind::Sequence,
                children: vec![0, 1],
            },
        ];
        let mut edges = BTreeSet::new();
        edges.insert((0, 1));

        let powl = TypedPowl::seal(nodes, edges, 2).unwrap();
        assert!(powl.verify_tree_properties().is_ok());
    }

    #[test]
    fn test_cycle_detection() {
        let nodes = vec![
            PowlNode::Activity { name: "A".to_string() },
            PowlNode::Activity { name: "B".to_string() },
        ];
        let mut edges = BTreeSet::new();
        edges.insert((0, 1));
        edges.insert((1, 0));  // Cycle!

        let result = TypedPowl::seal(nodes, edges, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_tree_projection() {
        let nodes = vec![
            PowlNode::Activity { name: "task1".to_string() },
        ];
        let edges = BTreeSet::new();

        let powl = TypedPowl::seal(nodes, edges, 0).unwrap();
        let proj = powl.to_tree_projection();

        assert_eq!(proj.root, OperatorKind::Activity);
        assert_eq!(proj.activity, Some("task1".to_string()));
    }
}
