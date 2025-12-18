use ariadne::{Color, Label, Report as AriadneReport, ReportKind, Source};
use parol_runtime::{LexerError, ParserError, SyntaxError};
use parol_runtime::{ParolError, Report};
use std::fs;
use std::ops::Range;
use std::path::Path;

#[derive(Debug)]
pub struct ErrorReporter;

impl ErrorReporter {
    pub fn report_error_with_content<T>(
        err: &ParolError,
        file_name: T,
        content: &str,
    ) -> anyhow::Result<()>
    where
        T: AsRef<Path>,
    {
        let path = file_name.as_ref();
        let file_n = path.to_string_lossy().to_string();
        let content = content.to_string();

        let report_lexer_error = |err: &LexerError| -> anyhow::Result<()> {
            match err {
                LexerError::TokenBufferEmptyError => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message("No valid token read")
                        .with_note("Token buffer is empty")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                LexerError::InternalError(e) => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message(format!("Internal lexer error: {e}"))
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                LexerError::LookaheadExceedsMaximum => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message("Lookahead exceeds maximum")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                LexerError::LookaheadExceedsTokenBufferLength => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message("Lookahead exceeds token buffer length")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                LexerError::ScannerStackEmptyError => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message("Tried to pop from empty scanner stack")
                        .with_note("Check balance of %push and %pop directives in your grammar")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                LexerError::RecoveryError(e) => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message(format!("Lexer recovery error: {e}"))
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
            }
            Ok(())
        };

        let report_parser_error = |err: &ParserError| -> anyhow::Result<()> {
            match err {
                ParserError::TreeError { source } => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message(format!("Error from syntree crate: {source}"))
                        .with_note("Internal error")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                ParserError::DataError(e) => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message(format!("Data error: {e}"))
                        .with_note("Error in generated source")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                ParserError::PredictionError { cause: _cause } => {
                    // This error is not reported because it is always followed by a syntax error
                    // and provides no additional information
                    // AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                    //     .with_message("Production prediction failed")
                    //     .with_note(cause)
                    //     .finish()
                    //     .eprint((file_n.clone(), Source::from(&content)))?
                }
                ParserError::SyntaxErrors { entries } => {
                    for SyntaxError {
                        cause,
                        error_location,
                        unexpected_tokens,
                        expected_tokens,
                        source,
                        ..
                    } in entries
                    {
                        if let Some(source) = source {
                            Self::report_error_with_content(source, file_name.as_ref(), &content)?;
                        }
                        let (range, unexpected_token): (Range<usize>, String) =
                            if let Some(unexpected_token) = unexpected_tokens.iter().next() {
                                (
                                    (&unexpected_token.token).into(),
                                    unexpected_token.token_type.clone(),
                                )
                            } else {
                                ((&**error_location).into(), "".to_string())
                            };

                        let mut report = AriadneReport::build(
                            ReportKind::Error,
                            (file_n.clone(), range.clone()),
                        )
                        .with_message("Syntax error")
                        .with_label(
                            Label::new((file_n.clone(), range))
                                .with_message(format!("Found `{}`", unexpected_token))
                                .with_color(Color::Red),
                        );

                        for un in unexpected_tokens.iter().skip(1) {
                            report = report.with_label(
                                Label::new((file_n.clone(), Into::<Range<usize>>::into(&un.token)))
                                    .with_message(format!("Found `{}`", un.token_type))
                                    .with_color(Color::Yellow),
                            );
                        }

                        report
                            .with_note(format!(
                                "Expecting one of {}",
                                expected_tokens
                                    .iter()
                                    .map(|e| format!("`{}`", e))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ))
                            .with_note(cause.to_string())
                            .finish()
                            .eprint((file_n.clone(), Source::from(&content)))?;
                    }
                }
                ParserError::UnprocessedInput { last_token, .. } => {
                    let un_span: Range<usize> = Into::<Range<usize>>::into(&**last_token);
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), un_span.clone()))
                        .with_message("Unprocessed input is left after parsing has finished")
                        .with_label(
                            Label::new((file_n.clone(), un_span))
                                .with_message("Unprocessed")
                                .with_color(Color::Red),
                        )
                        .with_note("Unprocessed input could be a problem in your grammar.")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?;
                }
                ParserError::Unsupported {
                    context,
                    error_location,
                } => {
                    let range: Range<usize> = (&*error_location.clone()).into();
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), range.clone()))
                        .with_message("Unsupported language feature")
                        .with_label(
                            Label::new((file_n.clone(), range))
                                .with_message("Unsupported")
                                .with_color(Color::Red),
                        )
                        .with_note(format!("Context: {context}"))
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?;
                }
                ParserError::InternalError(e) => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message(format!("Internal parser error: {e}"))
                        .with_note("This may be a bug. Please report it!")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                ParserError::TooManyErrors { count } => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message(format!("Too many errors: {count}"))
                        .with_note("The parser has stopped because too many errors occurred.")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
                ParserError::RecoveryFailed => {
                    AriadneReport::build(ReportKind::Error, (file_n.clone(), 0..0))
                        .with_message("Error recovery failed")
                        .with_note("The parser has stopped because error recovery failed.")
                        .finish()
                        .eprint((file_n.clone(), Source::from(&content)))?
                }
            }
            Ok(())
        };

        match err {
            ParolError::ParserError(e) => report_parser_error(e),
            ParolError::LexerError(e) => report_lexer_error(e),
            ParolError::UserError(e) => Self::report_user_error(e),
        }
    }

    pub fn report_user_error(err: &anyhow::Error) -> anyhow::Result<()> {
        AriadneReport::build(ReportKind::Error, ("User error", 0..0))
            .with_message("User error")
            .with_note(err.to_string())
            .with_note(
                err.source()
                    .map_or("No details".to_string(), |s| s.to_string()),
            )
            .finish()
            .eprint(("User error", Source::from("")))
            .map_err(|e| anyhow::anyhow!(e))
    }
}

impl Report for ErrorReporter {
    fn report_error<T>(err: &ParolError, file_name: T) -> anyhow::Result<()>
    where
        T: AsRef<Path>,
    {
        let path = file_name.as_ref();
        let content = fs::read_to_string(path).unwrap_or_default();
        Self::report_error_with_content(err, file_name, &content)
    }

    fn report_user_error(err: &anyhow::Error) -> anyhow::Result<()> {
        Self::report_user_error(err)
    }
}
