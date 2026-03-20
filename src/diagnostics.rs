use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Warning: {message}")]
#[diagnostic(severity(Warning))]
struct Warning {
    message: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Error: {message}")]
#[diagnostic(severity(Error))]
struct Err {
    message: String,
}

pub fn warn(msg: impl Into<String>) {
    eprintln!(
        "{:?}",
        miette::Report::new(Warning {
            message: msg.into()
        })
    );
}

pub fn err(msg: impl Into<String>) {
    eprintln!(
        "{:?}",
        miette::Report::new(Err {
            message: msg.into()
        })
    );
}

pub fn set_miette_hook() -> miette::Result<()> {
    miette::set_hook(Box::new(|_| {
        let mut theme = miette::GraphicalTheme::unicode();
        // the default warning character seems to render weirdly in some terminal emulators...
        theme.characters.warning = '!'.into();
        Box::new(miette::GraphicalReportHandler::new_themed(theme))
    }))
    .into_diagnostic()
}
