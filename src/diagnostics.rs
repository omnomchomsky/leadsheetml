#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub span: Option<SourceSpan>,
    pub code: Option<String>,
}

#[derive(Debug)]

pub enum LeadSheetMLError<R>{
    Pest(Box<pest::error::Error<R>>),
    Syntax {
        message: String,
        rule: Option<R>,
        span: Option<SourceSpan>
    },
    Internal{
        message: String,
        rule: Option<R>,
        span: Option<SourceSpan>
    },
}

pub type ParseResult<T, R> = Result<T, LeadSheetMLError<R>>;

pub fn internal_error<R>(
    message: impl Into<String>,
    rule: Option<R>,
    span: Option<pest::Span<'_>>,
) -> LeadSheetMLError<R> {
    LeadSheetMLError::Internal {
        message: message.into(),
        rule,
        span: span.map(SourceSpan::from_pest_span),
    }
}

pub fn internal<T, R>(message: impl Into<String>, rule: Option<R>, span: Option<pest::Span<'_>>) -> ParseResult<T,R> {
    Err(internal_error(message, rule, span))
}

pub fn syntax<T,R>(message: impl Into<String>, rule: Option<R>, span: Option<pest::Span<'_>>) -> ParseResult<T, R> {
    let span = span.map(SourceSpan::from_pest_span);
    Err(LeadSheetMLError::Syntax {
        message: message.into(),
        rule,
        span
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan{
    pub start: usize,
    pub end: usize
}

impl SourceSpan{
    pub fn from_pest_span(span: pest::Span<'_>) ->Self{
        Self{
            start: span.start(),
            end: span.end()
        }
    }
}
