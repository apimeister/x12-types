extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, GenericArgument, PathArguments, Type};

#[proc_macro_derive(DisplaySegment)]
pub fn display_segment(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = generate_segment_display(&ast).unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

#[proc_macro_derive(DisplayX12)]
pub fn display_x12(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = generate_x12(&ast).unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

#[proc_macro_derive(ParseSegment)]
pub fn parse_segment(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = generate_segment_parser(&ast).unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

/// Derives a transaction-set (or loop) parser from the struct definition.
///
/// This is the parsing counterpart to [`DisplayX12`]: it walks the struct's
/// fields in declaration order and emits the same `nom`-based parsing logic that
/// was previously hand-written for every transaction set.
///
/// Field kinds are mapped as follows:
/// - plain segment `T`         -> `T::parse` (mandatory, consumes one segment)
/// - `Option<T>`               -> `opt(T::parse)`
/// - `Vec<T>` (segment `T`)    -> `many0(T::parse)`
/// - `Vec<L>` with a loop attr -> `while peek(trigger) { L::parse }`
///
/// Loop fields must be annotated with the segment(s) that start the loop:
/// `#[x12(loop_trigger = "HL")]`, or `#[x12(loop_trigger = "L5|LH1")]` when more
/// than one segment can open the loop. The element type `L` is expected to also
/// derive `ParseX12` so its own body parses recursively.
#[proc_macro_derive(ParseX12, attributes(x12))]
pub fn parse_x12(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = generate_x12_parser(&ast).unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

fn generate_segment_display(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let output = gen_types(ast);
    let s = format_ident!("{}", name).to_string().to_uppercase();
    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt<'x>(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut str: Vec<String> = vec![];
                str.push(#s.to_string());
                #(#output)*
                let joined = str.join("*");
                let joined = joined.trim_end_matches("*");
                if joined.len() > 3 {
                    write!(f, "{}~\n", joined)
                }else{
                    write!(f, "")
                }
            }
        }
    })
}

fn generate_segment_parser(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let output = parse_types(ast);
    let s = format_ident!("{}", name).to_string().to_uppercase();
    Ok(quote! {
        impl<'a> crate::util::Parser<&'a str, #name, nom::error::Error<&'a str>> for #name {
            fn parse(input: &'a str) -> ::nom::IResult<&'a str, #name> {
                let (rest, vars) = crate::util::parse_line(input, #s)?;
                let mut obj = #name::default();
                #(#output)*
                Ok((rest, obj))
            }
        }
    })
}

fn gen_types(ast: &DeriveInput) -> Vec<TokenStream> {
    let x = &ast.data;
    let mut output = vec![];
    if let Data::Struct(s) = x {
        let f = &s.fields;
        for o in f {
            let id = o.ident.clone().unwrap();
            let t = &o.ty;
            if let syn::Type::Path(p) = t {
                let s = &p.path.segments;
                let w = &s.first().unwrap().ident;
                let ty = w.to_string();
                match ty.as_str() {
                    "Vec" => {
                        let ts = quote! {
                            if self.#id.is_empty() {
                                str.push("".to_string());
                            }else{
                                self.#id.iter().for_each(|x| str.push(format!("{}",x)));
                            }
                        };
                        output.push(ts);
                    }
                    "Option" => {
                        let ts = quote! {
                            str.push(self.#id.as_ref().map_or("".to_string(),|x| format!("{}",x)));
                        };
                        output.push(ts);
                    }
                    _ => {
                        let ts = quote! {
                            str.push(format!("{}",self.#id));
                        };
                        output.push(ts);
                    }
                }
            }
        }
    }
    output
}

/// Name of the single generic argument of a `Vec<..>` / `Option<..>` type, if any.
fn inner_type_name(ty: &Type) -> Option<String> {
    outer_and_inner(ty).and_then(|(_, inner)| {
        if let Type::Path(p) = inner {
            p.path.segments.last().map(|s| s.ident.to_string())
        } else {
            None
        }
    })
}

fn parse_types(ast: &DeriveInput) -> Vec<TokenStream> {
    let x = &ast.data;
    let mut output = vec![];
    if let Data::Struct(s) = x {
        let f = &s.fields;
        for (idx, o) in f.iter().enumerate() {
            let id = o.ident.clone().unwrap();
            let t = &o.ty;
            if let syn::Type::Path(p) = t {
                let s = &p.path.segments;
                let w = &s.first().unwrap().ident;
                let ty = w.to_string();
                match ty.as_str() {
                    "Vec" => {
                        let ts = quote! {
                            //TODO
                            obj.#id = "".to_string();
                        //     if self.#id.is_empty() {
                        //         str.push("".to_string());
                        //     }else{
                        //         self.#id.iter().for_each(|x| str.push(format!("{}",x)));
                        //     }
                        };
                        output.push(ts);
                    }
                    "Option" => {
                        // `Option<String>` keeps its original codegen; any other
                        // optional inner type is built via the `X12Element` trait.
                        let ts = if inner_type_name(t).as_deref() == Some("String") {
                            quote! {
                                obj.#id = vars.get(#idx).map(crate::util::unborrow_string);
                            }
                        } else {
                            quote! {
                                obj.#id = vars
                                    .get(#idx)
                                    .map(|x| crate::util::X12Element::from_x12(x));
                            }
                        };
                        output.push(ts);
                    }
                    "String" => {
                        let ts = quote! {
                            obj.#id = vars.get(#idx).unwrap().to_string();
                        };
                        output.push(ts);
                    }
                    // Any other plain (mandatory) field type is built via the
                    // `X12Element` trait, e.g. `Element<u32>`.
                    _ => {
                        let ts = quote! {
                            obj.#id = crate::util::X12Element::from_x12(vars.get(#idx).unwrap());
                        };
                        output.push(ts);
                    }
                }
            }
        }
    }
    output
}

/// Return the single generic argument of a `Vec<..>` / `Option<..>` type path,
/// along with the wrapper name ("Vec" / "Option"). Returns `None` for any other
/// type, which is treated as a plain mandatory segment.
fn outer_and_inner(ty: &Type) -> Option<(String, &Type)> {
    let Type::Path(p) = ty else { return None };
    let seg = p.path.segments.last()?;
    let name = seg.ident.to_string();
    if name != "Vec" && name != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    for arg in &args.args {
        if let GenericArgument::Type(inner) = arg {
            return Some((name, inner));
        }
    }
    None
}

/// Parse `#[x12(loop_trigger = "A")]` / `#[x12(loop_trigger = "A|B")]` on a field
/// into the list of trigger segment names. Empty when the field is not a loop.
fn loop_triggers(field: &Field) -> syn::Result<Vec<String>> {
    let mut triggers = Vec::new();
    for attr in &field.attrs {
        if !attr.path().is_ident("x12") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("loop_trigger") {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                triggers.extend(
                    lit.value()
                        .split('|')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty()),
                );
                Ok(())
            } else {
                Err(meta.error("unknown x12 attribute; expected `loop_trigger`"))
            }
        })?;
    }
    Ok(triggers)
}

fn generate_x12_parser(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let mut steps: Vec<TokenStream> = vec![];
    if let Data::Struct(s) = &ast.data {
        for field in &s.fields {
            let id = field.ident.clone().unwrap();
            let ty = &field.ty;
            let triggers = loop_triggers(field)?;
            if !triggers.is_empty() {
                // Loop field: `Vec<L>` driven by one or more trigger segments.
                let elem = match outer_and_inner(ty) {
                    Some((ref wrapper, elem)) if wrapper == "Vec" => elem,
                    _ => {
                        return Err(syn::Error::new_spanned(
                            ty,
                            "fields annotated with #[x12(loop_trigger = ..)] must be `Vec<_>`",
                        ))
                    }
                };
                let conds = triggers.iter().map(|t| {
                    let tid = format_ident!("{}", t);
                    quote! {
                        ::nom::combinator::peek(::nom::combinator::opt(#tid::parse))
                            .parse(loop_rest)?
                            .1
                            .is_some()
                    }
                });
                steps.push(quote! {
                    {
                        let mut items = ::std::vec::Vec::new();
                        let mut loop_rest = rest;
                        while #(#conds)||* {
                            let (r, item) = #elem::parse(loop_rest)?;
                            loop_rest = r;
                            items.push(item);
                        }
                        rest = loop_rest;
                        obj.#id = items;
                    }
                });
            } else if let Some((wrapper, elem)) = outer_and_inner(ty) {
                let combinator = if wrapper == "Option" {
                    quote! { ::nom::combinator::opt(#elem::parse) }
                } else {
                    quote! { ::nom::multi::many0(#elem::parse) }
                };
                steps.push(quote! {
                    let (r, v) = #combinator.parse(rest)?;
                    rest = r;
                    obj.#id = v;
                });
            } else {
                // Plain mandatory segment.
                steps.push(quote! {
                    let (r, v) = #ty::parse(rest)?;
                    rest = r;
                    obj.#id = v;
                });
            }
        }
    }
    Ok(quote! {
        impl<'a> crate::util::Parser<&'a str, #name, nom::error::Error<&'a str>> for #name {
            fn parse(input: &'a str) -> ::nom::IResult<&'a str, #name> {
                #[allow(unused_imports)]
                use ::nom::Parser as _;
                #[allow(unused_imports)]
                use crate::util::Parser as _;
                let mut obj = #name::default();
                #[allow(unused_mut)]
                let mut rest = input;
                #(#steps)*
                Ok((rest, obj))
            }
        }
    })
}

fn generate_x12(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let output = gen_types(ast);
    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt<'x>(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut str: Vec<String> = vec![];
                #(#output)*
                // filter empty lines
                let str: Vec<String> = str
                    .iter()
                    .map(|v| v.clone())
                    .filter(|s| !s.is_empty())
                    .collect();
                let joined = str.join("");
                write!(f, "{}", joined)
            }
        }
    })
}
