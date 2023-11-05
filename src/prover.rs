use parol_runtime::log::{debug, trace};
use petgraph::{
    algo::all_simple_paths,
    dot::{Config, Dot},
    graph::NodeIndex,
    prelude::DiGraph,
    visit::DfsPostOrder,
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
pub(crate) enum TransformationState {
    #[default]
    Unprocessed,
    Transformed,
    Closed,
}

impl Display for TransformationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            TransformationState::Unprocessed => write!(f, "?"),
            TransformationState::Transformed => write!(f, "✓"),
            TransformationState::Closed => write!(f, "✖"),
        }
    }
}

// The nodes of our proposition tree are propositions paired with a transformation state to indicate
// whether a node has been processed already or whether the branch is closed.
pub(crate) type PropositionTree = DiGraph<(Proposition, TransformationState), ()>;

/// The outcome of the prover algorithm for a specific proposition
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ProveResult {
    /// Default value
    /// Meaningless
    #[default]
    Undefined,

    /// Solve process is still ongoing
    Processing,

    /// *Tautology*
    /// The proposition is always TRUE independent from the values of its variables
    Proven,

    /// The propositions truth dependents on the values of its variables
    Contingent,

    /// *Contradiction*
    /// The proposition is always FALSE independent from the values of its variables
    Falsified,
}

impl Display for ProveResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            ProveResult::Undefined => write!(f, "Undefined"),
            ProveResult::Processing => write!(f, "Processing"),
            ProveResult::Proven => write!(f, "Logically True"),
            ProveResult::Contingent => write!(f, "Contingent"),
            ProveResult::Falsified => write!(f, "Logically False"),
        }
    }
}

#[derive(Debug)]
pub struct Prover {
    root: RefCell<NodeIndex>,
}

impl Default for Prover {
    fn default() -> Self {
        Self {
            root: RefCell::new(NodeIndex::end()),
        }
    }
}

impl Prover {
    pub fn new() -> Self {
        Self::default()
    }
    /// This function tries do prove the given proposition by assuming the opposite and then
    /// trying to find a contradiction. If a contradiction is found we have proven the logical
    /// truth of the proposition.
    ///
    /// The return value of this function in case of `OK` is [crate::prover::ProveResult].
    /// If the value is `SolveResult::Contradiction` then the proposition is L-TRUE.
    /// The return value in case of `Err` is detailed in [crate::errors::RaaError].
    ///
    pub fn prove(&self, proposition: &Proposition) -> Result<ProveResult> {
        let mut prove_result = self.try_prove(proposition, true)?;
        if prove_result == ProveResult::Contingent {
            prove_result = self.try_prove(proposition, false)?;
        }
        Ok(prove_result)
    }

    fn try_prove(&self, proposition: &Proposition, negated: bool) -> Result<ProveResult> {
        let mut prove_result = ProveResult::Processing;
        let mut graph = PropositionTree::new();
        self.init_proposition_tree(proposition, &mut graph, negated)?;
        while prove_result == ProveResult::Processing {
            trace!(
                "{}{:?}",
                if negated { "neg " } else { "" },
                Dot::with_attr_getters(
                    &graph,
                    &[Config::EdgeNoLabel, Config::NodeNoLabel],
                    &|_, _| { String::default() },
                    &|g, n| { format!("label = \"{} ({}, {})\"", g[n.0].0, n.0.index(), g[n.0].1) }
                )
            );
            prove_result = self.inner_prove(&mut graph, negated)?;
        }
        trace!(
            "{}{:?}",
            if negated { "neg " } else { "" },
            Dot::with_attr_getters(
                &graph,
                &[Config::EdgeNoLabel, Config::NodeNoLabel],
                &|_, _| { String::default() },
                &|g, n| { format!("label = \"{} ({}, {})\"", g[n.0].0, n.0.index(), g[n.0].1) }
            )
        );
        Ok(prove_result)
    }

    // We insert a (possibly negated) variant of our proposition and try to refute it later.
    fn init_proposition_tree(
        &self,
        proposition: &Proposition,
        graph: &mut PropositionTree,
        negate: bool,
    ) -> Result<()> {
        // The node id of the root is stored in `self.root`.
        *self.root.borrow_mut() = match proposition {
            Proposition::Void => Err(RaaError::VoidExpression)?,
            _ => graph.add_node((
                if negate {
                    Proposition::Negation(Negation {
                        inner: Box::new(proposition.clone()),
                    })
                } else {
                    proposition.clone()
                },
                TransformationState::default(),
            )),
        };
        Ok(())
    }

    fn transform(proposition: &Proposition) -> Result<(Vec<Proposition>, Vec<Proposition>)> {
        Ok(match proposition {
            Proposition::Void => Err(RaaError::VoidExpression)?,
            Proposition::Atom(a) => {
                debug!("Transfer Atom {a}");
                (vec![], vec![])
            }
            Proposition::Negation(n) => match &*n.inner {
                // Rule "Double negation"
                // A branch that contains a proposition in the form ¬¬A can be appended with A.
                Proposition::Negation(Negation { inner }) => {
                    debug!("Transfer double negation {n} =>");
                    debug!("    [{}]", inner);
                    debug!("    []");
                    (vec![(**inner).clone()], vec![])
                }
                // Rule "Negated biimplication"
                // A branch that contains a proposition in the form ¬(A <-> B) can be appended with
                // two new branches, one containing A and ¬B and one containing ¬A and B.
                Proposition::BiImplication(BiImplication { left, right }) => {
                    debug!("Transfer negated biimplication {n} =>");
                    debug!("    [{}, !{}]", left, right);
                    debug!("    [!{}, {}]", left, right);
                    (
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
                    )
                }
                // Rule "Negated implication"
                // A branch that contains a proposition in the form ¬(A -> B) can be appended
                // with A and ¬B.
                Proposition::Implication(Implication { left, right }) => {
                    debug!("Transfer negated implication {n} =>");
                    debug!("    [{}, !{}]", left, right);
                    debug!("    []");
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
                // Rule "Negated disjunction"
                // A branch that contains a proposition in the form ¬(A ∨ B) can be appended
                // with ¬A and ¬B.
                Proposition::Disjunction(Disjunction { left, right }) => {
                    debug!("Transfer negated disjunction {n} =>");
                    debug!("    [!{}, !{}]", left, right);
                    debug!("    []");
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
                // Rule "Negated conjunction"
                // A branch that contains a proposition in the form ¬(A ∧ B) can be appended
                // with two new branches ¬A and ¬B.
                Proposition::Conjunction(Conjunction { left, right }) => {
                    debug!("Transfer negated conjunction {n} =>");
                    debug!("    [!{}]", left);
                    debug!("    [!{}]", right);
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
            // Rule "Implication"
            // A branch that contains a proposition in the form A -> B can be appended with two
            // new branches ¬A and B.
            Proposition::Implication(Implication { left, right }) => {
                debug!("Transfer implication {proposition} =>");
                debug!("    [!{}]", left);
                debug!("    [{}]", right);
                (
                    vec![Proposition::Negation(Negation {
                        inner: Box::new((**left).clone()),
                    })],
                    vec![(**right).clone()],
                )
            }
            // Rule "BiImplication"
            // A branch that contains a proposition in the form A <-> B can be appended with two
            // new branches, one containing A and B and one containing ¬A and ¬B.
            Proposition::BiImplication(BiImplication { left, right }) => {
                debug!("Transfer biimplication {proposition} =>");
                debug!("    [{}, {}]", left, right);
                debug!("    [!{}, !{}]", left, right);
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
            // Rule "Disjunction"
            // A branch that contains a proposition in the form A ∨ B can be appended with two
            // new branches A and B.
            Proposition::Disjunction(Disjunction { left, right }) => {
                debug!("Transfer disjunction {proposition} =>");
                debug!("    [{}]", left);
                debug!("    [{}]", right);
                (vec![(**left).clone()], vec![(**right).clone()])
            }
            // Rule "Conjunction"
            // A branch that contains a proposition in the form A ∧ B can be appended with A and
            // B.
            Proposition::Conjunction(Conjunction { left, right }) => {
                debug!("Transfer conjunction {proposition} =>");
                debug!("    [{}, {}]", left, right);
                debug!("    []");
                (vec![(**left).clone(), (**right).clone()], vec![])
            }
        })
    }

    fn inner_prove(&self, graph: &mut PropositionTree, negated: bool) -> Result<ProveResult> {
        let mut changed = false;
        if let Some(unprocessed_node) = self.find_unprocessed_node(graph) {
            let (to_add_left, to_add_right) = Self::transform(&graph[unprocessed_node].0)?;
            let leafs_to_append = unclosed_leaf_nodes_of(graph, unprocessed_node);
            for leaf_node_id in leafs_to_append {
                let mut last_parent_node = leaf_node_id;
                for p in &to_add_left {
                    let new_node_id = graph.add_node((p.clone(), TransformationState::default()));
                    graph.add_edge(last_parent_node, new_node_id, ());
                    last_parent_node = new_node_id;
                }
                last_parent_node = leaf_node_id;
                for p in &to_add_right {
                    let new_node_id = graph.add_node((p.clone(), TransformationState::default()));
                    graph.add_edge(last_parent_node, new_node_id, ());
                    last_parent_node = new_node_id;
                }
            }
            graph[unprocessed_node].1 = TransformationState::Transformed;
            changed |= true;
        }
        if changed {
            self.update_branches_closed_state(graph)?;
        }
        self.check_all_branches_closed(graph, changed, negated)
    }

    fn update_branches_closed_state(&self, graph: &mut PropositionTree) -> Result<ProveResult> {
        let leaf_node_ids = leaf_nodes(graph);
        let mut leaf_nodes_to_close = vec![];
        for leaf_node_id in leaf_node_ids {
            let (_, transformation_state) = &graph[leaf_node_id];
            if *transformation_state == TransformationState::Closed {
                continue;
            }
            // We compare all nodes along the path from the root to this edge node with each other.
            let ancestors = self.ancestors(&*graph, leaf_node_id);
            let pairs = pairwise(&ancestors);
            if pairs.iter().any(|(i, j)| {
                let (a, _) = &graph[**i];
                let (b, _) = &graph[**j];
                match (a, b) {
                    (Proposition::Negation(n), _) => *n.inner == *b,
                    (_, Proposition::Negation(n)) => *n.inner == *a,
                    _ => false,
                }
            }) {
                leaf_nodes_to_close.push(leaf_node_id);
            }
        }
        for leaf_node_id in leaf_nodes_to_close {
            let (_, transformation_state) = &mut graph[leaf_node_id];
            *transformation_state = TransformationState::Closed;
        }
        Ok(ProveResult::Processing)
    }

    fn check_all_branches_closed(
        &self,
        graph: &PropositionTree,
        changed: bool,
        negated: bool,
    ) -> Result<ProveResult> {
        let leaf_node_ids = leaf_nodes(graph);
        let all_closed = leaf_node_ids
            .iter()
            .all(|i| graph[*i].1 == TransformationState::Closed);
        if all_closed {
            // This means all branches contain contradictions!
            Ok(if negated {
                // We used the negated proposition to refute it which indirectly proved it's truth.
                ProveResult::Proven
            } else {
                // We used the original proposition to refute it which directly falsified it.
                ProveResult::Falsified
            })
        } else if changed {
            // We need to continue until no branches can be developed anymore.
            Ok(ProveResult::Processing)
        } else {
            Ok(ProveResult::Contingent)
        }
    }

    fn ancestors(&self, graph: &PropositionTree, node_id: NodeIndex) -> Vec<NodeIndex> {
        let paths = all_simple_paths::<Vec<_>, _>(graph, *self.root.borrow(), node_id, 0, None)
            .collect::<Vec<_>>();
        // Tree constraint:
        // At most one path should exist from root to this end node.
        debug_assert!(paths.len() < 2, "length was {}", paths.len());
        if paths.is_empty() {
            vec![*self.root.borrow()]
        } else {
            // Path constraint:
            // The target node should be contained in the list of ancestors.
            debug_assert!(paths[0].contains(&node_id));
            paths[0].clone()
        }
    }

    fn find_unprocessed_node(&self, graph: &mut PropositionTree) -> Option<NodeIndex> {
        graph.node_indices().find(|i| {
            let node = &graph[*i];
            node.1 == TransformationState::Unprocessed
        })
    }
}

/// Generate pairs of elements in a slice without redundances.
fn pairwise<T>(v: &[T]) -> Vec<(&T, &T)> {
    let mut result = Vec::with_capacity(v.len());
    for (i, a) in v.iter().enumerate() {
        for b in v.iter().skip(i + 1) {
            result.push((a, b));
        }
    }
    result
}

fn leaf_nodes(graph: &PropositionTree) -> Vec<NodeIndex> {
    graph
        .node_indices()
        .filter(|i| graph.neighbors(*i).count() == 0)
        .collect::<Vec<NodeIndex>>()
}

fn unclosed_leaf_nodes_of(graph: &PropositionTree, start: NodeIndex) -> Vec<NodeIndex> {
    let mut dfs = DfsPostOrder::new(graph, start);
    let mut result = Vec::new();
    while let Some(i) = dfs.next(graph) {
        if graph[i].1 != TransformationState::Closed && graph.neighbors(i).count() == 0 {
            result.push(i)
        }
    }
    result
}

#[cfg(test)]
mod test {

    use crate::{
        conjunction::Conjunction,
        implication::Implication,
        proposition::Proposition,
        prover::{pairwise, ProveResult, Prover},
    };

    #[test]
    fn test_pairwise() {
        let v = vec!['i', 'j', 'k', 'l'];
        let pairs = pairwise(&v);
        assert_eq!(
            vec![
                (&'i', &'j'),
                (&'i', &'k'),
                (&'i', &'l'),
                (&'j', &'k'),
                (&'j', &'l'),
                (&'k', &'l')
            ],
            pairs
        );
    }

    #[test]
    fn test_solve() {
        // Logically True - Modus Ponens
        // p & (p -> q) -> q
        let proposition = Proposition::Implication(Implication {
            left: Box::new(Proposition::Conjunction(Conjunction {
                left: Box::new("p".into()),
                right: Box::new(Proposition::Implication(Implication {
                    left: Box::new("p".into()),
                    right: Box::new("q".into()),
                })),
            })),
            right: Box::new("q".into()),
        });
        assert_eq!(
            ProveResult::Proven,
            Prover::new().prove(&proposition).unwrap()
        );
    }
}
