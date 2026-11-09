#![doc = include_str!("./README.md")]

use proc_macro::{
    Delimiter,
    TokenStream,
    TokenTree
};

#[proc_macro_attribute]
#[doc = include_str!("./docs/syntax.md")]
pub fn syntax(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut element = Vec::<TokenTree>::new();
    let mut iterator = item.into_iter().fuse();
    loop {
        match [iterator.next(), iterator.next()] {
            [
                Some(TokenTree::Punct(punctuation)),
                Some(TokenTree::Group(group))
            ] if let Delimiter::Bracket = group.delimiter()
                && punctuation.as_char() == '#' =>
            {
                element.extend([punctuation.into(), group.into()]);
            }
            mut trees => {
                element.extend(
                    attribute
                        .into_iter()
                        .chain(trees.iter_mut().flat_map(Option::take))
                        .chain(iterator)
                );
                break element.into_iter().collect();
            }
        }
    }
}
