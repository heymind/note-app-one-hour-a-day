use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter::Iterator;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Error, FnArg, Ident, ItemFn, LitStr, PatType, Result, Token,
};

#[derive(Debug)]
struct RoutesAttr {
    methods: Punctuated<Ident, Token![,]>,
    path: LitStr,
}

impl Parse for RoutesAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let parser = Punctuated::<Ident, Token![,]>::parse_separated_nonempty;
        Ok(Self {
            methods: parser(input)?,
            path: input.parse()?,
        })
    }
}

impl RoutesAttr {
    fn methods(&self) -> Vec<String> {
        self.methods
            .iter()
            .map(|id| id.to_string().to_uppercase())
            .collect()
    }

    fn path(&self) -> String {
        self.path.value()
    }
}

fn transform(attr: RoutesAttr, mut input: ItemFn) -> Result<TokenStream> {
    let ident = &input.sig.ident;

    let mut extractors: Vec<TokenStream> = Vec::with_capacity(input.sig.inputs.len());
    for arg in input.sig.inputs.iter_mut() {
        if let FnArg::Typed(arg) = arg {
            if arg.attrs.len() != 1 || !arg.attrs[0].path.is_ident("extract") {
                Err(Error::new(
                    arg.span(),
                    "one extractor attribute should be provided",
                ))?;
            }
            let attr = arg.attrs.pop().unwrap();
            let ts = attr.parse_args::<TokenStream>()?;
            extractors.push(quote! {
                #ts(&mut req).await?
            });
        } else {
            unreachable!()
        }
    }

    let endpoint_id = format_ident!("{}_endpoint", ident);
    let extractor_iter = extractors.iter();
    let methods = attr.methods();
    let endpoint_path = attr.path();
    let endpoint_ts = quote! {
        static #endpoint_id: ::routes::Endpoint = ::routes::Endpoint::Single {
            path: #endpoint_path,
            methods: &[#(#methods),*],
            handle: ::std::lazy::SyncLazy::new(||{
                ::routes::Handle(Box::new(|mut req| {
                    Box::pin(async {
                        Ok::<::hyper::Response<::hyper::Body>,Box<(dyn ::std::error::Error + Send + Sync)>>(#ident(#(#extractor_iter),*).await?)
                    })
                }))
            })
        }
    };

    Ok(endpoint_ts)
}
#[proc_macro_attribute]
pub fn routes(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as RoutesAttr);
    let item_fn = parse_macro_input!(input as ItemFn);
    match transform(attr, item_fn) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_routes_attr() {
        let routes_attr: RoutesAttr = syn::parse_quote!(get,post "123$");
        assert_eq!(routes_attr.methods(), vec!["GET", "POST"]);
        assert_eq!(routes_attr.path(), "123$");
    }

    #[test]
    fn test_transform() {
        let routes_attr: RoutesAttr = syn::parse_quote!(get,post "123$");
        let sth = transform(
            routes_attr,
            syn::parse_quote! {
                async fn abc(#[extract(ccc(dd))] req:Request) -> anyhow::Result<()> {
                    //123
                }
            },
        );
        println!("{}", sth.unwrap());
    }
}
