use anyhow::Result;
use raa_tt::{
    proposition::Proposition,
    prover::{ProveResult, Prover},
    raa_tt_grammar::RaaTtGrammar,
    raa_tt_parser::parse,
};

pub const PROPOSITIONS: &[(&str, ProveResult)] = &[
    ("(p -> (!p & r))", ProveResult::Contingent),
    ("!p -> q", ProveResult::Contingent),
    ("p | q | !p", ProveResult::Proven),
    ("a -> b -> c -> d", ProveResult::Contingent),
    ("((p & q) -> p)", ProveResult::Proven),
    ("((p | q) -> (r -> p))", ProveResult::Contingent),
    ("p -> (q -> r | p)", ProveResult::Proven),
    ("p & q -> p", ProveResult::Proven),
    ("(p -> q) -> ((q -> r) -> (p -> r))", ProveResult::Proven),
    (
        "!((!p | !q) & (p <-> !r) -> !(q & r))",
        ProveResult::Contingent,
    ),
    ("p & q <-> !(p & q) | !q", ProveResult::Falsified),
    ("p & !p", ProveResult::Falsified),
    ("p | !p", ProveResult::Proven),
    (
        "(!p | !q) & (p <-> !r) -> !(q & r)",
        ProveResult::Contingent,
    ),
    ("!(p & q <-> !(p & q) | !q)", ProveResult::Proven),
    ("q & (p -> q) -> p", ProveResult::Contingent),
    ("p -> q -> (p & r -> p)", ProveResult::Proven),
    ("p & (!q -> !p) -> q", ProveResult::Proven),
    ("p & (p -> q) -> q", ProveResult::Proven),
    ("!q & (p -> q) -> !p", ProveResult::Proven),
    ("(p | q) & !p -> q", ProveResult::Proven),
    ("(p | q) & !q -> p", ProveResult::Proven),
    ("p -> (p | q)", ProveResult::Proven),
    ("p -> (q | p)", ProveResult::Proven),
    ("p & q -> p", ProveResult::Proven),
    ("p & q -> q", ProveResult::Proven),
    ("!!p -> p", ProveResult::Proven),
    ("((p -> q) & (q -> r)) -> (p -> r)", ProveResult::Proven),
    (
        "(p -> (q -> r)) -> ((p -> q) -> (p -> r))",
        ProveResult::Proven,
    ),
    (
        "((p -> q) & (r -> s) & (p | r)) -> q | s",
        ProveResult::Proven,
    ),
    (
        "(p -> q) & (r -> s) & (!q | !s) -> !p | !r",
        ProveResult::Proven,
    ),
    ("p -> p", ProveResult::Proven),
    ("!p -> (p -> q)", ProveResult::Proven),
    ("q -> (p -> q)", ProveResult::Proven),
    ("p & !p -> q", ProveResult::Proven),
    ("p -> (q | !q)", ProveResult::Proven),
];

#[test]
fn compile_and_prove() -> Result<()> {
    for (i, (p, r)) in PROPOSITIONS.iter().enumerate() {
        let mut raa_tt_grammar = RaaTtGrammar::new();
        let file_name = format!("example_{i}");
        parse(p, &file_name, &mut raa_tt_grammar)?;
        assert_eq!(1, raa_tt_grammar.raa_tt.as_ref().unwrap().raa_tt_list.len());
        let proposition: Proposition =
            (&*raa_tt_grammar.raa_tt.as_ref().unwrap().raa_tt_list[0].biconditional).into();
        let solver = Prover::new();
        let solve_result = solver.prove(&proposition)?;
        assert_eq!(*r, solve_result);
    }
    Ok(())
}
