use std::path::Path;

use indexmap::IndexMap;
use itertools::Itertools as _;

use crate::{filters, functions, markdown};

/// Allows creation of a [`FilterExample`] with the following syntax:
///
/// `function_example!(mix(base=base, blend=red, amount=0.5) => "#804040")`
macro_rules! function_example {
    ($name:ident($($key:ident = $value:tt),*) => $output:expr) => {
        $crate::templating::FunctionExample {
            inputs: {
                let mut map = indexmap::IndexMap::new();
                $(map.insert(stringify!($key).to_string(), stringify!($value).to_string());)*
                map
            },
            output: $output.to_string(),
        }
    };
}

/// Allows creation of a [`FilterExample`] with the following syntax:
///
/// `filter_example!(red | add(hue=30)) => "#ff6666")`
macro_rules! filter_example {
    ($value:tt | $name:ident => $output:expr) => {
        $crate::templating::FilterExample {
            value: stringify!($value).to_string(),
            inputs: indexmap::IndexMap::new(),
            output: $output.to_string(),
        }
    };
    ($value:tt | $name:ident($($key:ident = $arg_value:tt),*) => $output:expr) => {
        $crate::templating::FilterExample {
            value: stringify!($value).to_string(),
            inputs: {
                let mut map = indexmap::IndexMap::new();
                $(map.insert(stringify!($key).to_string(), stringify!($arg_value).to_string());)*
                map
            },
            output: $output.to_string(),
        }
    };
}

pub fn make_engine(template_directory: &Path) -> tera::Tera {
    let mut tera = tera::Tera::default();
    tera.register_filter("add", filters::add);
    tera.register_filter("sub", filters::sub);
    tera.register_filter("mod", filters::modify);
    tera.register_filter("urlencode_lzma", filters::urlencode_lzma);
    tera.register_filter("trunc", filters::trunc);
    tera.register_filter("mix", filters::mix);
    tera.register_filter("css_rgb", filters::css_rgb);
    tera.register_filter("css_rgba", filters::css_rgba);
    tera.register_filter("css_hsl", filters::css_hsl);
    tera.register_filter("css_hsla", filters::css_hsla);
    tera.register_function("if", functions::if_fn);
    tera.register_function("object", functions::object);
    tera.register_function("css_rgb", functions::css_rgb);
    tera.register_function("css_rgba", functions::css_rgba);
    tera.register_function("css_hsl", functions::css_hsl);
    tera.register_function("css_hsla", functions::css_hsla);
    tera.register_function(
        "read_file",
        functions::read_file_handler(template_directory.to_owned()),
    );
    tera
}

#[must_use]
pub fn all_functions() -> Vec<Function> {
    vec![
        Function {
            name: "if".to_string(),
            description: "Return one value if a condition is true, and another if it's false"
                .to_string(),
            examples: vec![
                function_example!(if(cond=true, t=1, f=0) => "1"),
                function_example!(if(cond=false, t=1, f=0) => "0"),
            ],
        },
        Function {
            name: "object".to_string(),
            description: "Create an object from the input".to_string(),
            examples: vec![
                function_example!(object(a=1, b=2) => "{a: 1, b: 2}"),
                function_example!(object(a=1, b=2) => "{a: 1, b: 2}"),
            ],
        },
        Function {
            name: "css_rgb".to_string(),
            description: "Convert a color to an RGB CSS string".to_string(),
            examples: vec![function_example!(css_rgb(color=red) => "rgb(210, 15, 57)")],
        },
        Function {
            name: "css_rgba".to_string(),
            description: "Convert a color to an RGBA CSS string".to_string(),
            examples: vec![function_example!(css_rgba(color=red) => "rgba(210, 15, 57, 1.00)")],
        },
        Function {
            name: "css_hsl".to_string(),
            description: "Convert a color to an HSL CSS string".to_string(),
            examples: vec![function_example!(css_hsl(color=red) => "hsl(347, 87%, 44%)")],
        },
        Function {
            name: "css_hsla".to_string(),
            description: "Convert a color to an HSLA CSS string".to_string(),
            examples: vec![function_example!(css_hsla(color=red) => "hsla(347, 87%, 44%, 1.00)")],
        },
        Function {
            name: "read_file".to_string(),
            description:
                "Read and include the contents of a file, path is relative to the template file"
                    .to_string(),
            examples: vec![function_example!(read_file(path="abc.txt") => "abc")],
        },
    ]
}

#[must_use]
pub fn all_filters() -> Vec<Filter> {
    vec![
        Filter {
            name: "add".to_string(),
            description: "Add a value to a color".to_string(),
            examples: vec![
                filter_example!(red | add(hue=30) => "#ff6666"),
                filter_example!(red | add(saturation=0.5) => "#ff6666"),
            ],
        },
        Filter {
            name: "sub".to_string(),
            description: "Subtract a value from a color".to_string(),
            examples: vec![
                filter_example!(red | sub(hue=30) => "#d30f9b"),
                filter_example!(red | sub(saturation=60) => "#8f5360"),
            ],
        },
        Filter {
            name: "mod".to_string(),
            description: "Modify a color".to_string(),
            examples: vec![
                filter_example!(red | mod(lightness=80) => "#f8a0b3"),
                filter_example!(red | mod(opacity=0.5) => "#d20f3980"),
            ],
        },
        Filter {
            name: "mix".to_string(),
            description: "Mix two colors together".to_string(),
            examples: vec![filter_example!(red | mix(color=base, amount=0.5) => "#e08097")],
        },
        Filter {
            name: "urlencode_lzma".to_string(),
            description: "Serialize an object into a URL-safe string with LZMA compression"
                .to_string(),
            examples: vec![
                filter_example!(red | urlencode_lzma => "#ff6666"),
                filter_example!(some_object | urlencode_lzma => "XQAAgAAEAAAAAAAAAABAqEggMAAAAA=="),
            ],
        },
        Filter {
            name: "trunc".to_string(),
            description: "Truncate a number to a certain number of places".to_string(),
            examples: vec![filter_example!(1.123456 | trunc(places=3) => "1.123")],
        },
        Filter {
            name: "css_rgb".to_string(),
            description: "Convert a color to an RGB CSS string".to_string(),
            examples: vec![filter_example!(red | css_rgb => "rgb(210, 15, 57)")],
        },
        Filter {
            name: "css_rgba".to_string(),
            description: "Convert a color to an RGBA CSS string".to_string(),
            examples: vec![filter_example!(red | css_rgba => "rgba(210, 15, 57, 1.00)")],
        },
        Filter {
            name: "css_hsl".to_string(),
            description: "Convert a color to an HSL CSS string".to_string(),
            examples: vec![filter_example!(red | css_hsl => "hsl(347, 87%, 44%)")],
        },
        Filter {
            name: "css_hsla".to_string(),
            description: "Convert a color to an HSLA CSS string".to_string(),
            examples: vec![filter_example!(red | css_hsla => "hsla(347, 87%, 44%, 1.00)")],
        },
    ]
}

#[derive(serde::Serialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub examples: Vec<FunctionExample>,
}

#[derive(serde::Serialize)]
pub struct Filter {
    pub name: String,
    pub description: String,
    pub examples: Vec<FilterExample>,
}

#[derive(serde::Serialize)]
pub struct FunctionExample {
    pub inputs: IndexMap<String, String>,
    pub output: String,
}

#[derive(serde::Serialize)]
pub struct FilterExample {
    pub value: String,
    pub inputs: IndexMap<String, String>,
    pub output: String,
}

impl markdown::TableDisplay for Function {
    fn table_headings() -> Box<[String]> {
        Box::new([
            "Name".to_string(),
            "Description".to_string(),
            "Examples".to_string(),
        ])
    }

    fn table_row(&self) -> Box<[String]> {
        Box::new([
            format!("`{}`", self.name),
            self.description.clone(),
            if self.examples.is_empty() {
                "None".to_string()
            } else {
                self.examples.first().map_or_else(String::new, |example| {
                    format!(
                        "`{name}({input})` ⇒ `{output}`",
                        name = self.name,
                        input = example
                            .inputs
                            .iter()
                            .map(|(k, v)| format!("{k}={v}"))
                            .join(", "),
                        output = example.output
                    )
                })
            },
        ])
    }
}

impl markdown::TableDisplay for Filter {
    fn table_headings() -> Box<[String]> {
        Box::new([
            "Name".to_string(),
            "Description".to_string(),
            "Examples".to_string(),
        ])
    }

    fn table_row(&self) -> Box<[String]> {
        Box::new([
            format!("`{}`", self.name),
            self.description.clone(),
            if self.examples.is_empty() {
                "None".to_string()
            } else {
                self.examples.first().map_or_else(String::new, |example| {
                    if example.inputs.is_empty() {
                        format!(
                            "`{value} \\| {name}` ⇒ `{output}`",
                            value = example.value,
                            name = self.name,
                            output = example.output
                        )
                    } else {
                        format!(
                            "`{value} \\| {name}({input})` ⇒ `{output}`",
                            value = example.value,
                            name = self.name,
                            input = example
                                .inputs
                                .iter()
                                .map(|(k, v)| format!("{k}={v}"))
                                .join(", "),
                            output = example.output
                        )
                    }
                })
            },
        ])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn function_example_with_single_arg() {
        let example = function_example!(mix(base=base) => "#804040");
        assert_eq!(example.inputs["base"], "base");
        assert_eq!(example.output, "#804040");
    }

    #[test]
    fn function_example_with_multiple_args() {
        let example = function_example!(mix(base=base, blend=red, amount=0.5) => "#804040");
        assert_eq!(example.inputs["base"], "base");
        assert_eq!(example.inputs["blend"], "red");
        assert_eq!(example.inputs["amount"], "0.5");
        assert_eq!(example.output, "#804040");
    }

    #[test]
    fn filter_example_with_no_args() {
        let example = filter_example!(red | add => "#ff6666");
        assert_eq!(example.value, "red");
        assert_eq!(example.inputs.len(), 0);
        assert_eq!(example.output, "#ff6666");
    }

    #[test]
    fn filter_example_with_single_arg() {
        let example = filter_example!(red | add(hue=30) => "#ff6666");
        assert_eq!(example.value, "red");
        assert_eq!(example.inputs["hue"], "30");
        assert_eq!(example.output, "#ff6666");
    }

    #[test]
    fn filter_example_with_multiple_args() {
        let example = filter_example!(red | add(hue=30, saturation=0.5) => "#ff6666");
        assert_eq!(example.value, "red");
        assert_eq!(example.inputs["hue"], "30");
        assert_eq!(example.inputs["saturation"], "0.5");
        assert_eq!(example.output, "#ff6666");
    }
}
