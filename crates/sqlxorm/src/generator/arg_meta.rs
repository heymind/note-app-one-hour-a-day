use indexmap::IndexMap;

use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Eq},
    Ident, Lit, LitStr, Result, Token,
};
#[derive(Debug, Default)]
pub struct ArgMeta(IndexMap<String, Vec<Lit>>);

pub enum ArgMetaIndex<'a> {
    Index(usize),
    Named(&'a str),
}

impl<'a> Into<ArgMetaIndex<'a>> for &'a str {
    fn into(self) -> ArgMetaIndex<'a> {
        ArgMetaIndex::Named(self)
    }
}

impl<'a> Into<ArgMetaIndex<'a>> for usize {
    fn into(self) -> ArgMetaIndex<'a> {
        ArgMetaIndex::Index(self)
    }
}

impl ArgMeta {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn _get(&self, index: ArgMetaIndex) -> &[Lit] {
        match index {
            ArgMetaIndex::Index(idx) => self.0.get_index(idx).map(|(_, v)| v),
            ArgMetaIndex::Named(name) => self.0.get(name),
        }
        .map(|values| values.as_slice())
        .unwrap_or_else(|| &[])
    }
    pub fn has<'a>(&self, index: impl Into<ArgMetaIndex<'a>>) -> bool {
        if let Some(lit) = self._get(index.into()).first() {
            match lit {
                Lit::Bool(v) if v.value == false => false,
                _ => true,
            }
        } else {
            false
        }
    }
    pub fn get_bool<'a>(&self, index: impl Into<ArgMetaIndex<'a>>) -> Option<bool> {
        if let Some(Lit::Bool(b)) = self._get(index.into()).first() {
            Some(b.value)
        } else {
            None
        }
    }
    pub fn is_true<'a>(&self, index: impl Into<ArgMetaIndex<'a>>) -> bool {
        self.get_bool(index).filter(|x| *x == true).is_some()
    }

    pub fn get_str<'a>(&self, index: impl Into<ArgMetaIndex<'a>>) -> Option<String> {
        if let Some(Lit::Str(s)) = self._get(index.into()).first() {
            Some(s.value())
        } else {
            None
        }
    }
    pub fn get_strings<'a>(&self, index: impl Into<ArgMetaIndex<'a>>) -> Vec<String> {
        self._get(index.into())
            .iter()
            .filter_map(|lit| {
                if let Lit::Str(s) = lit {
                    Some(s.value())
                } else {
                    None
                }
            })
            .collect()
    }
}
impl Parse for ArgMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        fn parse_pair(input: ParseStream) -> Result<(Option<Ident>, Vec<Lit>)> {
            let mut key = None;
            if input.lookahead1().peek(Ident) {
                key = Some(input.parse()?);
                Eq::parse(input)?;
            }
            let values = if input.lookahead1().peek(Bracket) {
                let content;
                let _ = bracketed!(content in input);
                let values = Punctuated::<Lit, Token![,]>::parse_terminated(&content)?;
                values.into_pairs().map(|p| p.into_value()).collect()
            } else {
                vec![Lit::parse(input)?]
            };

            Ok((key, values))
        }

        let args = Punctuated::<(Option<Ident>, Vec<Lit>), Token![,]>::parse_terminated_with(
            input, parse_pair,
        )?;
        let mut inner = IndexMap::with_capacity(args.len());

        for (i, (key, values)) in args.into_pairs().map(|p| p.into_value()).enumerate() {
            let key = key
                .map(|id| id.to_string())
                .unwrap_or_else(|| i.to_string());
            inner.insert(key, values);
        }
        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_parse_arg_meta() {
        let arg_meta: ArgMeta = parse_quote!("asv", a = "sdsd", c = [123, 2, 3, "d"]);
        dbg!(&arg_meta);
        println!("{:?}", arg_meta.get_str("a"));
    }
}
