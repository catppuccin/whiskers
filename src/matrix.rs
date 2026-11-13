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

/// Iterates over every combination, with the last axis changing fastest.
pub fn combinations(matrix: &Matrix) -> impl Iterator<Item = Vec<(&str, &str)>> {
    let axes = matrix.iter().collect::<Vec<_>>();
    let mut positions = vec![0; axes.len()];
    let mut has_next = axes.iter().all(|(_, values)| !values.is_empty());

    std::iter::from_fn(move || {
        if !has_next {
            return None;
        }

        let combination = axes
            .iter()
            .zip(&positions)
            .map(|((key, values), &position)| (key.as_str(), values[position].as_str()))
            .collect();

        has_next = false;
        for ((_, values), position) in axes.iter().zip(&mut positions).rev() {
            *position += 1;
            if *position < values.len() {
                has_next = true;
                break;
            }
            *position = 0;
        }

        Some(combination)
    })
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

#[cfg(test)]
mod tests {
    use super::{combinations, Matrix};

    #[test]
    fn iterates_matrix_product() {
        let matrix = Matrix::from([
            ("flavor".into(), vec!["latte".into(), "mocha".into()]),
            ("variant".into(), vec!["light".into(), "dark".into()]),
        ]);
        let mut combinations = combinations(&matrix)
            .map(|mut combination| {
                combination.sort_unstable();
                combination
            })
            .collect::<Vec<_>>();
        combinations.sort_unstable();

        assert_eq!(
            combinations,
            [
                vec![("flavor", "latte"), ("variant", "dark")],
                vec![("flavor", "latte"), ("variant", "light")],
                vec![("flavor", "mocha"), ("variant", "dark")],
                vec![("flavor", "mocha"), ("variant", "light")],
            ]
        );
    }

    #[test]
    fn handles_empty_axes() {
        assert_eq!(combinations(&Matrix::new()).collect::<Vec<_>>(), [vec![]]);
        let matrix = Matrix::from([("variant".into(), vec![])]);
        assert_eq!(combinations(&matrix).count(), 0);
    }
}
