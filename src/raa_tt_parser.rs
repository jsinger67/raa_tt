// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans};
use parol_runtime::{ParolError, ParseTree, TerminalIndex};
use parol_runtime::{ScannerConfig, TokenStream, Tokenizer};
use std::path::Path;

use crate::raa_tt_grammar::RaaTtGrammar;
use crate::raa_tt_grammar_trait::RaaTtGrammarAuto;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[(&str, Option<(bool, &str)>); 14] = &[
    /*  0 */ (UNMATCHABLE_TOKEN, None),
    /*  1 */ (UNMATCHABLE_TOKEN, None),
    /*  2 */ (UNMATCHABLE_TOKEN, None),
    /*  3 */ (UNMATCHABLE_TOKEN, None),
    /*  4 */ (UNMATCHABLE_TOKEN, None),
    /*  5 */ (r"!", None),
    /*  6 */ (r"\&", None),
    /*  7 */ (r"\|", None),
    /*  8 */ (r"\->", None),
    /*  9 */ (r"<\->", None),
    /* 10 */ (r"\(", None),
    /* 11 */ (r"\)", None),
    /* 12 */ (r"[a-z][_a-zA-Z0-9]*", None),
    /* 13 */ (ERROR_TOKEN, None),
];

pub const TERMINAL_NAMES: &[&str; 14] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Not",
    /*  6 */ "And",
    /*  7 */ "Or",
    /*  8 */ "Cond",
    /*  9 */ "BiCond",
    /* 10 */ "LPar",
    /* 11 */ "RPar",
    /* 12 */ "Var",
    /* 13 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[TerminalIndex; 8]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r"//.*(\r\n|\r|\n)?",
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,  /* Not */
        6,  /* And */
        7,  /* Or */
        8,  /* Cond */
        9,  /* BiCond */
        10, /* LPar */
        11, /* RPar */
        12, /* Var */
    ],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 20] = &[
    /*  0 */ "And",
    /*  1 */ "BiCond",
    /*  2 */ "Biconditional",
    /*  3 */ "BiconditionalList",
    /*  4 */ "Cond",
    /*  5 */ "Conditional",
    /*  6 */ "ConditionalList",
    /*  7 */ "Conjunction",
    /*  8 */ "ConjunctionList",
    /*  9 */ "Disjunction",
    /* 10 */ "DisjunctionList",
    /* 11 */ "Factor",
    /* 12 */ "LPar",
    /* 13 */ "Negation",
    /* 14 */ "Not",
    /* 15 */ "Or",
    /* 16 */ "RPar",
    /* 17 */ "RaaTt",
    /* 18 */ "RaaTtList",
    /* 19 */ "Var",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 20] = &[
    /* 0 - "And" */
    LookaheadDFA {
        prod0: 1,
        transitions: &[],
        k: 0,
    },
    /* 1 - "BiCond" */
    LookaheadDFA {
        prod0: 4,
        transitions: &[],
        k: 0,
    },
    /* 2 - "Biconditional" */
    LookaheadDFA {
        prod0: 11,
        transitions: &[],
        k: 0,
    },
    /* 3 - "BiconditionalList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 13),
            Trans(0, 5, 2, 13),
            Trans(0, 9, 1, 12),
            Trans(0, 10, 2, 13),
            Trans(0, 11, 2, 13),
            Trans(0, 12, 2, 13),
        ],
        k: 1,
    },
    /* 4 - "Cond" */
    LookaheadDFA {
        prod0: 3,
        transitions: &[],
        k: 0,
    },
    /* 5 - "Conditional" */
    LookaheadDFA {
        prod0: 14,
        transitions: &[],
        k: 0,
    },
    /* 6 - "ConditionalList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 16),
            Trans(0, 5, 2, 16),
            Trans(0, 8, 1, 15),
            Trans(0, 9, 2, 16),
            Trans(0, 10, 2, 16),
            Trans(0, 11, 2, 16),
            Trans(0, 12, 2, 16),
        ],
        k: 1,
    },
    /* 7 - "Conjunction" */
    LookaheadDFA {
        prod0: 20,
        transitions: &[],
        k: 0,
    },
    /* 8 - "ConjunctionList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 22),
            Trans(0, 5, 2, 22),
            Trans(0, 6, 1, 21),
            Trans(0, 7, 2, 22),
            Trans(0, 8, 2, 22),
            Trans(0, 9, 2, 22),
            Trans(0, 10, 2, 22),
            Trans(0, 11, 2, 22),
            Trans(0, 12, 2, 22),
        ],
        k: 1,
    },
    /* 9 - "Disjunction" */
    LookaheadDFA {
        prod0: 17,
        transitions: &[],
        k: 0,
    },
    /* 10 - "DisjunctionList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 19),
            Trans(0, 5, 2, 19),
            Trans(0, 7, 1, 18),
            Trans(0, 8, 2, 19),
            Trans(0, 9, 2, 19),
            Trans(0, 10, 2, 19),
            Trans(0, 11, 2, 19),
            Trans(0, 12, 2, 19),
        ],
        k: 1,
    },
    /* 11 - "Factor" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 5, 2, 25), Trans(0, 10, 3, 26), Trans(0, 12, 1, 24)],
        k: 1,
    },
    /* 12 - "LPar" */
    LookaheadDFA {
        prod0: 5,
        transitions: &[],
        k: 0,
    },
    /* 13 - "Negation" */
    LookaheadDFA {
        prod0: 23,
        transitions: &[],
        k: 0,
    },
    /* 14 - "Not" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 15 - "Or" */
    LookaheadDFA {
        prod0: 2,
        transitions: &[],
        k: 0,
    },
    /* 16 - "RPar" */
    LookaheadDFA {
        prod0: 6,
        transitions: &[],
        k: 0,
    },
    /* 17 - "RaaTt" */
    LookaheadDFA {
        prod0: 8,
        transitions: &[],
        k: 0,
    },
    /* 18 - "RaaTtList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 10),
            Trans(0, 5, 1, 9),
            Trans(0, 10, 1, 9),
            Trans(0, 12, 1, 9),
        ],
        k: 1,
    },
    /* 19 - "Var" */
    LookaheadDFA {
        prod0: 7,
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 27] = &[
    // 0 - Not: '!';
    Production {
        lhs: 14,
        production: &[ParseType::T(5)],
    },
    // 1 - And: '&';
    Production {
        lhs: 0,
        production: &[ParseType::T(6)],
    },
    // 2 - Or: '|';
    Production {
        lhs: 15,
        production: &[ParseType::T(7)],
    },
    // 3 - Cond: '->';
    Production {
        lhs: 4,
        production: &[ParseType::T(8)],
    },
    // 4 - BiCond: '<->';
    Production {
        lhs: 1,
        production: &[ParseType::T(9)],
    },
    // 5 - LPar: '(';
    Production {
        lhs: 12,
        production: &[ParseType::T(10)],
    },
    // 6 - RPar: ')';
    Production {
        lhs: 16,
        production: &[ParseType::T(11)],
    },
    // 7 - Var: /[a-z][_a-zA-Z0-9]*/;
    Production {
        lhs: 19,
        production: &[ParseType::T(12)],
    },
    // 8 - RaaTt: RaaTtList /* Vec */;
    Production {
        lhs: 17,
        production: &[ParseType::N(18)],
    },
    // 9 - RaaTtList: Biconditional RaaTtList;
    Production {
        lhs: 18,
        production: &[ParseType::N(18), ParseType::N(2)],
    },
    // 10 - RaaTtList: ;
    Production {
        lhs: 18,
        production: &[],
    },
    // 11 - Biconditional: Conditional BiconditionalList /* Vec */;
    Production {
        lhs: 2,
        production: &[ParseType::N(3), ParseType::N(5)],
    },
    // 12 - BiconditionalList: BiCond^ /* Clipped */ Conditional BiconditionalList;
    Production {
        lhs: 3,
        production: &[ParseType::N(3), ParseType::N(5), ParseType::N(1)],
    },
    // 13 - BiconditionalList: ;
    Production {
        lhs: 3,
        production: &[],
    },
    // 14 - Conditional: Disjunction ConditionalList /* Vec */;
    Production {
        lhs: 5,
        production: &[ParseType::N(6), ParseType::N(9)],
    },
    // 15 - ConditionalList: Cond^ /* Clipped */ Disjunction ConditionalList;
    Production {
        lhs: 6,
        production: &[ParseType::N(6), ParseType::N(9), ParseType::N(4)],
    },
    // 16 - ConditionalList: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 17 - Disjunction: Conjunction DisjunctionList /* Vec */;
    Production {
        lhs: 9,
        production: &[ParseType::N(10), ParseType::N(7)],
    },
    // 18 - DisjunctionList: Or^ /* Clipped */ Conjunction DisjunctionList;
    Production {
        lhs: 10,
        production: &[ParseType::N(10), ParseType::N(7), ParseType::N(15)],
    },
    // 19 - DisjunctionList: ;
    Production {
        lhs: 10,
        production: &[],
    },
    // 20 - Conjunction: Factor ConjunctionList /* Vec */;
    Production {
        lhs: 7,
        production: &[ParseType::N(8), ParseType::N(11)],
    },
    // 21 - ConjunctionList: And^ /* Clipped */ Factor ConjunctionList;
    Production {
        lhs: 8,
        production: &[ParseType::N(8), ParseType::N(11), ParseType::N(0)],
    },
    // 22 - ConjunctionList: ;
    Production {
        lhs: 8,
        production: &[],
    },
    // 23 - Negation: Not^ /* Clipped */ Factor;
    Production {
        lhs: 13,
        production: &[ParseType::N(11), ParseType::N(14)],
    },
    // 24 - Factor: Var;
    Production {
        lhs: 11,
        production: &[ParseType::N(19)],
    },
    // 25 - Factor: Negation;
    Production {
        lhs: 11,
        production: &[ParseType::N(13)],
    },
    // 26 - Factor: LPar^ /* Clipped */ Biconditional RPar^ /* Clipped */;
    Production {
        lhs: 11,
        production: &[ParseType::N(16), ParseType::N(2), ParseType::N(12)],
    },
];

static SCANNERS: Lazy<Vec<ScannerConfig>> = Lazy::new(|| {
    vec![ScannerConfig::new(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
        &[],
    )]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut RaaTtGrammar<'t>,
) -> Result<ParseTree, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        17,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    llk_parser.trim_parse_tree();
    llk_parser.disable_recovery();

    // Initialize wrapper
    let mut user_actions = RaaTtGrammarAuto::new(user_actions);
    llk_parser.parse(
        TokenStream::new(input, file_name, &SCANNERS, MAX_K).unwrap(),
        &mut user_actions,
    )
}
