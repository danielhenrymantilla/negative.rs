//! Crate not intended for direct use.
//! Use https:://docs.rs/negative instead.
// Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template
#![allow(nonstandard_style, unused_imports)]

use ::core::{
    mem,
    ops::Not as _,
};
use ::proc_macro::{
    TokenStream,
};
use ::proc_macro2::{*,
    TokenStream as TokenStream2,
};
use ::quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use ::syn::{*,
    parse::{Parse, Parser, ParseStream},
    punctuated::Punctuated,
    Result, // Explicitly shadow it
    spanned::Spanned,
};

///
#[proc_macro_attribute] pub
fn negative_impl(
    args: TokenStream,
    input: TokenStream,
) -> TokenStream
{
    negative_impl_impl(args.into(), input.into())
    //  .map(|ret| { println!("{}", ret); ret })
        .unwrap_or_else(|err| {
            let mut errors =
                err .into_iter()
                    .map(|err| Error::new(
                        err.span(),
                        format_args!("`#[negative::negative_impl]`: {}", err),
                    ))
            ;
            let mut err = errors.next().unwrap();
            errors.for_each(|cur| err.combine(cur));
            err.combine(Error::new(
                Span::mixed_site(),
                "expected syntax: `[unsafe] impl<…> !Trait for Type [where …] {}`",
            ));
            err.to_compile_error()
        })
        .into()
}

struct ItemImpl {
    mb_unsafe: Option<Token![unsafe]>,
    impl_: Token![impl],
    generics: Generics,
    bang_: Token![!],
    Trait: Path,
    for_: Token![for],
    Receiver: Type,
    where_: Option<Token![where]>,
    rest: Vec<TokenTree>,
}

impl Parse for ItemImpl {
    fn parse(input: ParseStream<'_>)
      -> Result<Self>
    {
        let input = ItemImpl {
            mb_unsafe: input.parse()?,
            impl_: input.parse()?,
            generics: input.parse()?,
            bang_: input.parse()?,
            Trait: input.parse()?,
            for_: input.parse()?,
            Receiver: input.parse()?,
            where_: input.parse()?,
            rest: input.parse::<TokenStream2>()?.into_iter().collect(),
        };
        let last = input.rest.last().ok_or_else(|| Error::new(
            Span::mixed_site(),
            "unexpected end of input",
        ))?;
        if  matches!(
                last,
                TokenTree::Group(g)
                if g.delimiter() == Delimiter::Brace
                && g.stream().is_empty()
            )
            .not()
        {
            return Err(Error::new_spanned(
                last,
                "expected `{}`",
            ));
        }
        Ok(input)
    }
}

fn negative_impl_impl(
    args: TokenStream2,
    input: TokenStream2,
) -> Result<TokenStream2>
{
    // By default deny any attribute present.
    let _: parse::Nothing = parse2(args)?;

    let ItemImpl {
        mb_unsafe,
        impl_,
        generics,
        bang_,
        Trait,
        for_,
        Receiver,
        where_,
        rest,
    } = parse2(input)?;

    let bang_span = bang_.span();
    let where_ = where_.unwrap_or_else(|| Token![where](bang_span));
    let trivially_false_extra_where_predicate = quote_spanned!(bang_span=>
        for<'ඞ /* ' */> [()] : ::core::marker::Sized,
    );

    Ok(quote!(
        #mb_unsafe
        #impl_ #generics #Trait #for_ #Receiver
        #where_
            #trivially_false_extra_where_predicate
        #(#rest)*
    ))
}
