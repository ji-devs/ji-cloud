use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream, TokenTree};
use quote::quote;

pub struct PathPartsInput {
    pub name: Ident,
    pub path: String,
    pub parts: Vec<Ident>,
}

impl From<PathPartsInput> for TokenStream {
    fn from(value: PathPartsInput) -> TokenStream {
        let PathPartsInput {
            name,
            mut path,
            parts,
        } = value;

        let parts_placeholders = parts
            .iter()
            .map(|part| {
                let display_name = part.to_string().to_case(Case::Snake);
                let placeholder = format!("{{{display_name}}}");
                placeholder
            })
            .collect_vec();

        for part_placeholder in parts_placeholders.iter() {
            path = path.replacen("{}", &part_placeholder, 1);
        }

        let parts_replace = parts_placeholders
            .iter()
            .enumerate()
            .map(|(i, part_placeholder)| {
                // usize_unsuffixed needed for tuple index https://github.com/rust-lang/rust/issues/59553#issue-427261901
                let index = Literal::usize_unsuffixed(i);
                quote! {
                    src = src.replacen(#part_placeholder, &self.#index.get_path_string(), 1);
                }
            })
            .collect_vec();

        quote!(
            #[allow(missing_docs)]
            #[derive(Clone, Debug)]
            pub struct #name(#(pub #parts, )*);

            impl crate::api::endpoints::PathParts for #name {
                const PATH: &'static str = #path;

                fn get_filled(&self) -> String {
                    let mut src = String::from(Self::PATH);

                    #(#parts_replace)*

                    src
                }
            }
        )
    }
}

impl TryFrom<TokenStream> for PathPartsInput {
    type Error = anyhow::Error;

    fn try_from(value: TokenStream) -> Result<PathPartsInput, Self::Error> {
        let input: Vec<TokenTree> = value.into_iter().collect();

        let name = match input.get(0) {
            None => return Err(anyhow::anyhow!("Not enough input")),
            Some(name) => match name {
                TokenTree::Ident(name) => name.clone().into(),
                _ => return Err(anyhow::anyhow!("Expected Ident at first token")),
            },
        };

        ensure_is_arrow(&input, 1)?;

        let path = match input.get(3) {
            None => return Err(anyhow::anyhow!("Not enough input")),
            Some(name) => match name {
                TokenTree::Literal(path) => {
                    // remove quotes from string literal, i.e. first and last item
                    let mut path = path.to_string();
                    path.pop();
                    path.remove(0);
                    path
                }
                _ => return Err(anyhow::anyhow!("Expected string literal at fourth token")),
            },
        };

        ensure_is_arrow(&input, 1)?;

        let mut parts = vec![];

        for i in (6..input.len()).step_by(2) {
            let part = match &input[i] {
                TokenTree::Ident(name) => name.clone(),
                _ => return Err(anyhow::anyhow!("Expected Ident at this token")),
            };

            parts.push(part);
        }

        if path.matches("{}").count() != parts.len() {
            panic!(
                "Number of path placeholders ({{}}) doesn't match number of parts (impl PathPart)"
            );
        }

        let result = PathPartsInput { name, path, parts };

        Ok(result)
    }
}

fn ensure_is_arrow(input: &[TokenTree], index: usize) -> anyhow::Result<()> {
    match input.get(index) {
        None => return Err(anyhow::anyhow!("Not enough input")),
        Some(name) => {
            let not_arrow = Err(anyhow::anyhow!(
                "Expected '=' of arrow ('=>') at token index {index}"
            ));
            match name {
                TokenTree::Punct(punct) => {
                    if punct.as_char() != '=' {
                        return not_arrow;
                    }
                }
                _ => return not_arrow,
            }
        }
    };

    let index = index + 1;
    match input.get(index) {
        None => return Err(anyhow::anyhow!("Not enough input")),
        Some(name) => {
            let not_arrow = Err(anyhow::anyhow!(
                "Expected '>' of arrow ('=>') at token index {index}"
            ));
            match name {
                TokenTree::Punct(punct) => {
                    if punct.as_char() != '>' {
                        return not_arrow;
                    }
                }
                _ => return not_arrow,
            }
        }
    };

    Ok(())
}
