use std::collections::HashMap;

use catppuccin::FlavorName;

pub type Matrix = HashMap<String, Vec<String>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown magic iterable: {name}")]
    UnknownIterable { name: String },

    #[error("Invalid matrix array object element: must have a single key and an array of strings as value")]
    InvalidObjectElement,

    #[error("Invalid matrix array element: must be a string or object")]
    InvalidElement,
}

// matrix in frontmatter is a list of strings or objects.
// objects must have a single key and an array of strings as the value.
// string array elements are substituted with the array from `iterables`.
pub fn from_values(
    values: Vec<tera::Value>,
    only_flavor: Option<FlavorName>,
) -> Result<Matrix, Error> {
    let iterables = magic_iterables(only_flavor);
    values
        .into_iter()
        .map(|v| match v {
            tera::Value::String(s) => {
                let iterable = iterables
                    .get(s.as_str())
                    .ok_or_else(|| Error::UnknownIterable { name: s.clone() })?;
                Ok((s, iterable.clone()))
            }
            tera::Value::Object(o) => {
                let (key, value) = o.into_iter().next().ok_or(Error::InvalidObjectElement)?;
                let value: Vec<String> =
                    tera::from_value(value).map_err(|_| Error::InvalidObjectElement)?;
                Ok((key, value))
            }
            _ => Err(Error::InvalidElement),
        })
        .collect::<Result<Matrix, Error>>()
}

fn magic_iterables(only_flavor: Option<FlavorName>) -> HashMap<&'static str, Vec<String>> {
    HashMap::from([
        (
            "flavor",
            only_flavor.map_or_else(
                || {
                    catppuccin::PALETTE
                        .into_iter()
                        .map(|flavor| flavor.identifier().to_string())
                        .collect::<Vec<String>>()
                },
                |flavor| vec![flavor.identifier().to_string()],
            ),
        ),
        ("accent", ctp_accents()),
    ])
}

fn ctp_accents() -> Vec<String> {
    catppuccin::PALETTE
        .latte
        .colors
        .iter()
        .filter(|c| c.accent)
        .map(|c| c.name.identifier().to_string())
        .collect()
}
