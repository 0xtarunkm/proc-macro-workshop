use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input};

#[derive(Debug)]
struct SeqMacroInput {
    var: syn::Ident,
    from: syn::Lit,
    to: syn::Lit,
    body: proc_macro2::TokenStream,
}

impl Parse for SeqMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let var = syn::Ident::parse(input)?;
        let _in = <syn::Token![in]>::parse(input)?;
        let from = syn::Lit::parse(input)?;
        let _dots = <syn::Token![..]>::parse(input)?;
        let to = syn::Lit::parse(input)?;
        
        let content;
        let _braces = syn::braced!(content in input);
        let body = proc_macro2::TokenStream::parse(&content)?;

        Ok(SeqMacroInput { var, from, to, body })
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as SeqMacroInput);

    // println!("{:#?}", input);

    TokenStream::new()
}
