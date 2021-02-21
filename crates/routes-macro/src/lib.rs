use either::Either;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::Iterator;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Brace,
    Error, FnArg, Ident, ItemFn, LitStr, Path, Result, Token, Visibility,
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
    let mut ident: Ident = parse_quote!(origin);
    std::mem::swap(&mut input.sig.ident, &mut ident);
    let vis = &input.vis;
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

    let extractor_iter = extractors.iter();
    let methods = attr.methods();
    let endpoint_path = attr.path();

    let origin_sig = &input.sig;
    let origin_block = &input.block;
    let ts = quote! {
        #vis mod #ident {
            use super::*;
            pub #origin_sig #origin_block
            pub async fn handle(mut req: ::hyper::Request<::hyper::Body>) -> ::std::result::Result<::hyper::Response<::hyper::Body>,Box<(dyn ::std::error::Error + Send + Sync)>> {
                Ok::<::hyper::Response<::hyper::Body>,Box<(dyn ::std::error::Error + Send + Sync)>>(origin(#(#extractor_iter),*).await?)
            }
            pub static endpoint: ::routes::Endpoint = ::routes::Endpoint::single(&[#(#methods),*],#endpoint_path,||{
                ::routes::Handle(Box::new(|req|Box::pin(async{handle(req).await})))
            });
        }

    };
    Ok(ts)
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
/// pub routes_group!(something  "abc" {
///     hello, // a endpoint
///     "/a" => world, // a mount    
/// .   "/b" => {hello,world, "/d" => hello} // nested mounts
//})
// mod something {
//     pub static endpoint: ::routes::Endpoint = ::routes::Endpoint::group(
//         "abc",
//          []
//     )
// }
#[derive(Debug)]
struct RoutesGroup {
    vis: Visibility,
    ident: Ident,
    prefix: Option<LitStr>,
    brace: Brace,
    item: RoutesGroupItem,
}
impl Parse for RoutesGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            vis: input.parse()?,
            ident: input.parse()?,
            prefix: input.parse()?,
            brace: braced!(content in input),
            item: content.parse()?,
        })
    }
}
impl RoutesGroup {
    fn transform(self) -> Result<TokenStream> {
        let Self {
            vis,
            ident,
            prefix,
            item,
            ..
        } = self;
        let prefix = prefix.map(|v| v.value()).unwrap_or_default();
        let item_ts = item.transform()?;
        Ok(quote! {
            #vis mod #ident {
                use super::*;
                pub static endpoint: ::routes::Endpoint = ::routes::Endpoint::group(#prefix,#item_ts);
            }
        })
    }
}

#[derive(Debug)]
struct RoutesGroupItem(
    Punctuated<(Option<(LitStr, Token![=>])>, Either<Path, (Brace, Self)>), Token![,]>,
);

impl RoutesGroupItem {
    fn transform(self) -> Result<TokenStream> {
        let mut items = Vec::with_capacity(self.0.len());
        for item in self.0 {
            let item = match item {
                (None, Either::Left(path)) => quote! {&#path::endpoint},
                (None, Either::Right((_, sub))) => {
                    let sub_ts = sub.transform()?;
                    quote! {
                        {static endpoint: ::routes::Endpoint = ::routes::Endpoint::group("",#sub_ts);&endpoint}
                    }
                }
                (Some((prefix, _)), Either::Left(path)) => {
                    quote! {
                        {static endpoint: ::routes::Endpoint = ::routes::Endpoint::group(#prefix,&[&#path::endpoint]);&endpoint}
                    }
                }
                (Some((prefix, _)), Either::Right((_, sub))) => {
                    let sub_ts = sub.transform()?;
                    quote! {
                        {static endpoint: ::routes::Endpoint = ::routes::Endpoint::group(#prefix,#sub_ts);&endpoint}
                    }
                }
            };
            items.push(item);
        }
        Ok(quote! {
            &[#(#items),*]
        })
    }
}
impl Parse for RoutesGroupItem {
    fn parse(input: ParseStream) -> Result<Self> {
        fn parse_inner(
            input: ParseStream,
        ) -> Result<(
            Option<(LitStr, Token![=>])>,
            Either<Path, (Brace, RoutesGroupItem)>,
        )> {
            let mut prefix = None;
            let endpoint;
            if input.lookahead1().peek(LitStr) {
                prefix = Some((input.parse()?, input.parse()?));
            }
            let ahead = input.lookahead1();
            if ahead.peek(Brace) {
                let content;
                endpoint = Either::Right((braced!(content in input), content.parse()?));
            } else {
                endpoint = Either::Left(input.parse()?);
            }
            Ok((prefix, endpoint))
        }
        Ok(Self(Punctuated::parse_terminated_with(input, parse_inner)?))
    }
}

#[proc_macro]
pub fn routes_group(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse_macro_input!(input as RoutesGroup).transform() {
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
    fn test_parse_routes_group_item() {
        let item: RoutesGroupItem = syn::parse_quote!(get,post,"here" => get,"there" => {get,post});
        dbg!(item);
    }

    #[test]
    fn test_parse_routes_group() {
        let item: RoutesGroup =
            syn::parse_quote!(abc "sss" {get,post,"here" => get,"there" => {get,post}});
        dbg!(item);
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
