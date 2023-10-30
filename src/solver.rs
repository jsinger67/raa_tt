use petgraph::{
    algo::all_simple_paths, graph::NodeIndex, prelude::DiGraph, visit::EdgeRef, Direction::Incoming,
};
use std::{
    cell::RefCell,
    fmt::{Display, Error, Formatter},
    vec,
};

use crate::{
    bi_implication::BiImplication,
    conjunction::Conjunction,
    disjunction::Disjunction,
    errors::{RaaError, Result},
    implication::Implication,
    negation::Negation,
    proposition::Proposition,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SolveResult {
    #[default]
    Undefined,
    // Solve process is still ongoing
    Solving,
    //
    Proofed,
    FalsifiedOrContingent,
}

impl From<bool> for SolveResult {
    fn from(value: bool) -> Self {
        if value {
            SolveResult::Proofed
        } else {
            SolveResult::FalsifiedOrContingent
        }
    }
}

impl Display for SolveResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            SolveResult::Undefined => write!(f, "Undefined"),
            SolveResult::Solving => write!(f, "Solving"),
            SolveResult::Proofed => write!(f, "Proofed"),
            SolveResult::FalsifiedOrContingent => write!(f, "Falsified or contingent"),
        }
    }
}

pub(crate) struct Solver {
    root: RefCell<NodeIndex<u32>>,
}

impl Solver {
    pub(crate) fn new() -> Self {
        Self {
            root: RefCell::new(NodeIndex::end()),
        }
    }
    /// The [solve] function tries do proof the given proposition by assuming the opposite
    /// and then trying to find a contradiction. If a contradiction is found we have proven the
    /// logical truth of the proposition.
    ///
    /// The return value of this function in case of `OK` is [crate::solver::SolveResult].
    /// If the value is `SolveResult::Contradiction` then the proposition is L-TRUE.
    /// The return value in case of `Err` is detailed in [crate::errors::RaaError].
    ///
    pub(crate) fn solve(&self, proposition: &Proposition) -> Result<SolveResult> {
        // The nodes of our proposition tree are propositions paired with a boolean to indicate
        // whether the branch is closed (true means closed).
        // This boolean value is only of interest if it's node is a leaf node.
        let mut graph = DiGraph::<(Proposition, bool), ()>::new();
        self.init_proposition_tree(proposition, &mut graph)?;
        let mut solve_result = SolveResult::Solving;
        while solve_result == SolveResult::Solving {
            solve_result = self.inner_solve(&mut graph)?;
        }
        Ok(solve_result)
    }

    // We insert a negated variant of our proposition and try to refute it later.
    // The node id of the root is stored in `self.root`.
    fn init_proposition_tree(
        &self,
        proposition: &Proposition,
        graph: &mut DiGraph<(Proposition, bool), ()>,
    ) -> Result<()> {
        *self.root.borrow_mut() = match proposition {
            Proposition::Void => Err(RaaError::VoidExpression)?,
            _ => graph.add_node((
                Proposition::Negation(Negation {
                    inner: Box::new(proposition.clone()),
                }),
                false,
            )),
        };
        Ok(())
    }

    fn transform(proposition: &Proposition) -> Result<(Vec<Proposition>, Vec<Proposition>)> {
        Ok(match proposition {
            Proposition::Void => Err(RaaError::VoidExpression)?,
            Proposition::Atom(_) => (vec![], vec![]),
            Proposition::Negation(n) => match &*n.inner {
                // Rule "Double negation"
                // A branch that contains a proposition in the form ¬¬A can be appended with A.
                Proposition::Negation(Negation { inner }) => (vec![(**inner).clone()], vec![]),
                // Rule "Negated biimplication"
                // A branch that contains a proposition in the form ¬(A <-> B) can be appended with
                // two new branches, one containing A and ¬B and one containing ¬A and B.
                Proposition::BiImplication(BiImplication { left, right }) => (
                    vec![
                        (**left).clone(),
                        Proposition::Negation(Negation {
                            inner: Box::new((**right).clone()),
                        }),
                    ],
                    vec![
                        Proposition::Negation(Negation {
                            inner: Box::new((**left).clone()),
                        }),
                        (**right).clone(),
                    ],
                ),
                Proposition::Implication(Implication { left, right }) => {
                    // Rule "Negated implication"
                    // A branch that contains a proposition in the form ¬(A -> B) can be appended
                    // with A and ¬B.
                    (
                        vec![
                            (**left).clone(),
                            Proposition::Negation(Negation {
                                inner: Box::new((**right).clone()),
                            }),
                        ],
                        vec![],
                    )
                }
                Proposition::Disjunction(Disjunction { left, right }) => {
                    // Rule "Negated disjunction"
                    // A branch that contains a proposition in the form ¬(A ∨ B) can be appended
                    // with ¬A and ¬B.
                    (
                        vec![
                            Proposition::Negation(Negation {
                                inner: Box::new((**left).clone()),
                            }),
                            Proposition::Negation(Negation {
                                inner: Box::new((**right).clone()),
                            }),
                        ],
                        vec![],
                    )
                }
                Proposition::Conjunction(Conjunction { left, right }) => {
                    // Rule "Negated conjunction"
                    // A branch that contains a proposition in the form ¬(A ∧ B) can be appended
                    // with two new branches ¬A and ¬B.
                    (
                        vec![Proposition::Negation(Negation {
                            inner: Box::new((**left).clone()),
                        })],
                        vec![Proposition::Negation(Negation {
                            inner: Box::new((**right).clone()),
                        })],
                    )
                }
                // Otherwise no changes
                _ => (vec![], vec![]),
            },
            Proposition::Implication(Implication { left, right }) => {
                // Rule "Implication"
                // A branch that contains a proposition in the form A -> B can be appended with two
                // new branches ¬A and B.
                (
                    vec![Proposition::Negation(Negation {
                        inner: Box::new((**left).clone()),
                    })],
                    vec![(**right).clone()],
                )
            }
            Proposition::BiImplication(BiImplication { left, right }) => {
                // Rule "BiImplication"
                // A branch that contains a proposition in the form A <-> B can be appended with two
                // new branches, one containing A and B and one containing ¬A and ¬B.
                (
                    vec![(**left).clone(), (**right).clone()],
                    vec![
                        Proposition::Negation(Negation {
                            inner: Box::new((**left).clone()),
                        }),
                        Proposition::Negation(Negation {
                            inner: Box::new((**right).clone()),
                        }),
                    ],
                )
            }
            Proposition::Disjunction(Disjunction { left, right }) => {
                // Rule "Disjunction"
                // A branch that contains a proposition in the form A ∨ B can be appended with two
                // new branches A and B.
                (vec![(**left).clone()], vec![(**right).clone()])
            }
            Proposition::Conjunction(Conjunction { left, right }) => {
                // Rule "Conjunction"
                // A branch that contains a proposition in the form A ∧ B can be appended with A and
                // B.
                (vec![(**left).clone(), (**right).clone()], vec![])
            }
        })
    }

    fn inner_solve(&self, graph: &mut DiGraph<(Proposition, bool), ()>) -> Result<SolveResult> {
        let leaf_node_ids = leaf_nodes(graph);
        let mut changed = false;
        for leaf_node_id in leaf_node_ids {
            let (end_node_proposition, end_node_closed) = &graph[leaf_node_id];
            if *end_node_closed {
                continue;
            }
            let (to_add_left, to_add_right) = Self::transform(end_node_proposition)?;
            let mut last_parent_node = leaf_node_id;
            for p in to_add_left {
                let new_node_id = graph.add_node((p, false));
                graph.add_edge(last_parent_node, new_node_id, ());
                last_parent_node = new_node_id;
                changed |= true;
            }
            last_parent_node = leaf_node_id;
            for p in to_add_right {
                let new_node_id = graph.add_node((p, false));
                graph.add_edge(last_parent_node, new_node_id, ());
                last_parent_node = new_node_id;
                changed |= true;
            }
        }
        if changed {
            self.update_branches_closed_state(graph)?;
        }
        self.check_all_branches_closed(graph, changed)
    }

    fn update_branches_closed_state(
        &self,
        graph: &mut DiGraph<(Proposition, bool), ()>,
    ) -> Result<SolveResult> {
        let leaf_node_ids = leaf_nodes(graph);
        let root = self.root.borrow();
        let mut leaf_nodes_to_close = vec![];
        for leaf_node_id in leaf_node_ids {
            let (leaf_node_proposition, leaf_node_closed) = &graph[leaf_node_id];
            if *leaf_node_closed {
                continue;
            }
            // We compare all nodes along the path from the root to this edge node with the two last
            // nodes which possibly have been appended during the last transformation step.
            // We use the negated value to be able to identify pairs of (A, ¬A) in the branch.
            let comparison = match *leaf_node_proposition {
                Proposition::Negation(ref n) => (*n.inner).clone(),
                _ => (*leaf_node_proposition).clone(),
            };
            let (comparison_2, comparison_2_id) = graph
                .edges_directed(leaf_node_id, Incoming)
                .find(|e| e.target() == leaf_node_id)
                .map_or((Proposition::Void, NodeIndex::end()), |e| {
                    (graph[e.source()].0.clone(), e.source())
                });
            let comparison_2 = match comparison_2 {
                Proposition::Negation(n) => *n.inner,
                _ => comparison_2,
            };
            for p in all_simple_paths::<Vec<_>, _>(&*graph, *root, leaf_node_id, 1, None) {
                if p.iter().any(|i| {
                    graph[*i].0 == comparison
                        || (*i != comparison_2_id && graph[*i].0 == comparison_2)
                }) {
                    leaf_nodes_to_close.push(leaf_node_id);
                }
            }
        }
        for leaf_node_id in leaf_nodes_to_close {
            let (_, leaf_node_closed) = &mut graph[leaf_node_id];
            *leaf_node_closed = true;
        }
        Ok(SolveResult::Solving)
    }

    fn check_all_branches_closed(
        &self,
        graph: &DiGraph<(Proposition, bool), ()>,
        changed: bool,
    ) -> Result<SolveResult> {
        let leaf_node_ids = leaf_nodes(graph);
        let all_closed = leaf_node_ids.iter().all(|i| graph[*i].1);
        if all_closed {
            Ok(SolveResult::Proofed)
        } else if changed {
            Ok(SolveResult::Solving)
        } else {
            Ok(SolveResult::FalsifiedOrContingent)
        }
    }
}

fn leaf_nodes(graph: &DiGraph<(Proposition, bool), ()>) -> Vec<NodeIndex<u32>> {
    graph
        .node_indices()
        .filter(|i| graph.neighbors(*i).count() == 0)
        .collect::<Vec<NodeIndex<u32>>>()
}
