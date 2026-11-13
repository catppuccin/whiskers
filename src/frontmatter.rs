use std::collections::HashMap;

#[derive(Debug)]
pub struct Document {
    pub frontmatter: HashMap<String, tera::Value>,
    pub body: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid YAML frontmatter (L{line}:{column}) : {message}")]
    InvalidYaml {
        line: usize,
        column: usize,
        message: String,
    },
}

pub fn parse(input: &str) -> Result<Document, Error> {
    let Some((frontmatter, body)) = split(input) else {
        // no frontmatter to parse
        return Ok(Document {
            frontmatter: HashMap::new(),
            body: input.to_string(),
        });
    };

    Ok(Document {
        frontmatter: serde_yaml::from_str(frontmatter).map_err(|e| Error::InvalidYaml {
            line: e.location().map_or(0, |l| l.line()),
            column: e.location().map_or(0, |l| l.column()),
            message: e.to_string(),
        })?,
        body: body.to_string(),
    })
}

fn split(template: &str) -> Option<(&str, &str)> {
    // we consider a template to possibly have frontmatter iff:
    // * line 0 is "---"
    // * there is another "---" on another line
    let template = template.trim_start();
    let sep = frontmatter_separator(template);

    template
        .strip_prefix(sep)?
        .split_once(sep)
        .map(|(a, b)| (a.trim(), b))
}

fn frontmatter_separator(template: &str) -> &'static str {
    // Prefer the most common line ending in the template, defaulting to LF on ties.
    let crlf = template.matches("\r\n").count();
    let cr = template.matches('\r').count() - crlf;
    let lf = template.matches('\n').count() - crlf;
    if crlf > cr && crlf > lf {
        "---\r\n"
    } else if cr > crlf && cr > lf {
        "---\r"
    } else {
        "---\n"
    }
}

#[cfg(test)]
mod tests {
    use super::split;

    #[test]
    fn splits_frontmatter_with_supported_line_endings() {
        for ending in ["\n", "\r\n", "\r"] {
            let body = format!("body{ending}");
            let template = format!("---{ending}a: b{ending}---{ending}{body}");
            assert_eq!(split(&template), Some(("a: b", body.as_str())));
        }
    }

    #[test]
    fn uses_dominant_line_ending_and_defaults_to_lf_on_ties() {
        assert_eq!(
            split("---\nkey: value\n---\nx\r\ny\r\nz\r\n"),
            Some(("key: value", "x\r\ny\r\nz\r\n"))
        );
        assert_eq!(split("---\nkey: value\n---\nx\r\ny\r\nz\r\nw\r\n"), None);
    }
}
