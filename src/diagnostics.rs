use miette::{IntoDiagnostic, MietteDiagnostic, Report, Severity};

pub fn warn(msg: impl Into<String>) {
    let diagnostic =
        MietteDiagnostic::new(format!("Warning: {}", msg.into())).with_severity(Severity::Warning);
    eprintln!("{:?}", Report::new(diagnostic));
}

pub fn err(msg: impl Into<String>) {
    let diagnostic =
        MietteDiagnostic::new(format!("Error: {}", msg.into())).with_severity(Severity::Error);
    eprintln!("{:?}", Report::new(diagnostic));
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
