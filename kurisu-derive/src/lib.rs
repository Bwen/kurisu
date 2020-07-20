//! Provides `derive(Kurisu)` for the [Kurisu](../kurisu/index.html) crate.
//!
//! The Kurisu derive macro is not meant to do any validation nor display usage for the command line
//! string values. It will only construct **_[kurisu::Arg](../kurisu/arg/index.html)_**
//! structs according to a given Rust Struct, and put them within a function
//! `get_info_instance` wrapped by a [once_cell](https://docs.rs/once_cell) static instance of
//! a **_[kurisu::Info](../kurisu/struct.Info.html)_** struct.
//!
//! Example:
//! ```ignore
//! use kurisu::*;
//!
//! #[derive(Debug, Kurisu)]
//! struct Yargs {
//!     knots: usize,
//! }
//!
//! fn main() {
//!     let env_vars: Vec<String> = std::env::args().skip(1).collect();
//!     # let env_vars: Vec<String> = vec!["--knots 8".to_string()];
//!
//!     // The Derive Macro add `from_args` function to the user's struct,
//!     // `from_args` will normalize the command line values into kurisu::Info.env_args
//!     let args = Yargs::from_args(&env_vars);
//!     
//!     let knots;
//!     // This scope is so the mutex guard gets released after
//!     // we retrieve the information we need.
//!     {
//!         // The Derive Macro also add `get_info_instance` function to the user's struct.
//!         // The Kurisu:Info struct holds all the parsed information the Derive Macro did.
//!         let info = Yargs::get_info_instance(&env_vars).lock().unwrap();
//!         knots = info.args.iter().find(|a| a.name == "knots").unwrap().clone();
//!     }
//!     
//!     // If we assume the current example was called as follow: `mycli --knots 8`
//!     // We would find that information within the Kurisu::Info struct
//!     assert_eq!(info.env_args, vec!["--knots=8".to_string()]);
//!     assert_eq!(knots.value, vec!["8".to_string()]);
//!     assert_eq!(args.knots, 8);
//! }
//! ```
//!
//! ## Rust types with specific meaning
//!
//! Type          | Description
//! --------------|---------------------------------------------
//! bool          | Defines a Flag
//! u8            | Defines a Flag with occurrence tracking as its value
//! String        | Defines value type of an Option or Argument
//! PathBuf       | Defines value type of an Option or Argument
//! usize         | Defines value type of an Option or Argument
//! isize         | Defines value type of an Option or Argument
//! f64           | Defines value type of an Option or Argument
//! Vec<T>        | Defines a repetitive Option or Argument
//!
//! ## Override default behaviors on main Struct
//!
//! The struct that drives these behaviors is handled by
//! *__[kurisu::Info](../kurisu/struct.Info.html)__*, most of the annotation are associated to one
//! of its field.
//!
//! Field name    | Default | Annotation | Description
//! --------------|--------------|-------------------------|---------------------------------------------
//! name          | "Unknown"    | name = "mycli"          | Change name on usage screen
//! version       | 0            | version = "0.1.0"       | Change version on usage screen
//! desc          | None         | desc = "some short text"| Change description on usage screen
//! doc           | None         | ///                     | Change DISCUSSION text on usage screen
//! allow_noargs  | false        | allow_noargs            | Does not display usage screen if no command line values
//! &nbsp;        | &nbsp;       | cargo                   | Will try to fetch name, version & desc from the Cargo.toml, the specific field annotation take precedence on the cargo annotation
//! &nbsp;        | &nbsp;       | nosort                  | Avoid sorting alphabetically arguments, flags & options on usage screen
//! &nbsp;        | &nbsp;       | auto_shorts             | Enables the auto generation of short flags / options according to their field's name first letter, by default no short flags / options are generated unless specified on the struct's field annotation
//!
//! Example:
//! ```ignore
//!     #[derive(Debug, Kurisu)]
//!     // Only picks the version from Cargo.toml since name & desc are already present
//!     #[kurisu(name = "yargs", cargo, desc = "some desc here", auto_shorts)]
//!     /// This text is for the DISCUSSION
//!     /// section in the use screen
//!     ///
//!     /// It can be a very large text
//!     struct Yargs {}
//! ```
//!
//! ## Override default behaviors on main Struct's fields
//!
//! The struct that drives these behaviors is handled by
//! *__[kurisu::Arg](../kurisu/arg/struct.Arg.html)__*, most of the annotation are associated to one
//! of its field.
//!
//! Field name    | Default       | Annotation                 | Description
//! --------------|---------------|----------------------------|---------------------------------------------
//! vname         | field's name  | vname = "myname"           | Change the name of the <VALUE> in the usage screen
//! position      | None          | pos **OR** pos = "1"       | Defines the position of an argument, `pos` without a value defines an infinite positional argument
//! doc           | None          | ///                        | Defines the description of arg on the usage screen
//! short         | None          | short **OR** short = "b"   | Change the letter used by short flag / option, otherwise it takes the first letter of the struct's field name
//! long          | field's name  | nolong **OR** long = "myname" | Change or remove the long flag / option
//! env           | field's name  | env = "MYSQL_HOST"         | By default the field's name is used to look for environment variable match as last resort, this is to change which environment variable is being matched
//! env_prefix    | None          | env_prefix = "MYSQL_"      | Will take the field's name and prefix it and look for a match environment variable
//! required_if   | None          | required_if = "fieldname"  | Will make this option required if the other flag/option is present
//! default       | ""            | default = "42"             | If not present in the command line values, struct field will be assigned this default value instead of default type value
//! exit          | None          | exit = "my_exit_func"      | Local function that triggers std::process::exit() after being executed. A bit like the usage display, it will stop execution at `from_args`
//! &nbsp;        | &nbsp;        | parse_with = "my_func"     | Local function that allows custom parsing for this argument/flag/option
//!
//! Example:
//! ```ignore
//! use std::path::PathBuf;
//!
//! #[derive(Debug, Kurisu)]
//! struct Yargs {
//!     #[kurisu(pos = 1)]
//!     file: PathBuf,
//!     #[kurisu(pos)] // Infinite positional argument
//!     others: String,
//!     #[kurisu(short, nolong, env_prefix = "MYSQL_")]
//!     host: String,
//!     #[kurisu(long = "very-long-flag")]
//!     more: bool,
//!     /// description for `crashed` on usage screen
//!     crashed: bool,
//! }
//! ```
//!

#![forbid(unsafe_code)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Attribute, Field, NestedMeta};

#[proc_macro_derive(Kurisu, attributes(kurisu))]
pub fn kurisu_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_kurisu_macro(&ast)
}

fn get_fields_named(data: &syn::Data) -> &syn::FieldsNamed {
    match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => &fields,
        _ => abort_call_site!("kurisu macro only supports non-tuple structs"),
    }
}

fn meta_attributes(attrs: &[Attribute]) -> Vec<(proc_macro2::Ident, Option<syn::Lit>)> {
    let mut doc_lines: Vec<String> = Vec::new();
    let mut attributes: Vec<(proc_macro2::Ident, Option<syn::Lit>)> = Vec::new();
    for attr in attrs.iter() {
        if attr.path.is_ident("doc") {
            if let Ok(meta) = attr.parse_meta() {
                if let syn::Meta::NameValue(syn::MetaNameValue { ref lit, .. }) = meta {
                    let doc = quote! {#lit}.to_string().replace('"', "");
                    doc_lines.push(doc.trim().to_string());
                }
            }

            continue;
        } else if !attr.path.is_ident("kurisu") {
            continue;
        }

        if let Ok(meta) = attr.parse_meta() {
            if let syn::Meta::NameValue(syn::MetaNameValue { ref path, ref lit, .. }) = meta {
                attributes.push((path.get_ident().unwrap().clone(), Some(lit.clone())));
            } else if let syn::Meta::List(syn::MetaList { ref nested, .. }) = meta {
                for item in nested.iter() {
                    if let NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue { ref path, ref lit, .. })) = item {
                        attributes.push((path.get_ident().unwrap().clone(), Some(lit.clone())));
                    } else if let NestedMeta::Meta(syn::Meta::Path(ref path)) = item {
                        attributes.push((path.get_ident().unwrap().clone(), None));
                    }
                }
            }
        }
    }

    if !doc_lines.is_empty() {
        attributes.push((
            proc_macro2::Ident::new("doc", proc_macro2::Span::call_site()),
            Some(syn::Lit::Str(syn::LitStr::new(
                doc_lines.join("\n").trim(),
                proc_macro2::Span::call_site(),
            ))),
        ));
    }

    attributes
}

fn meta_value(name: &str, attrs: &[(proc_macro2::Ident, Option<syn::Lit>)], option_wrap: bool) -> Option<proc_macro2::TokenStream> {
    attrs.iter().find_map(|(ident, value)| {
        if ident.eq(name) {
            if value.is_some() {
                if option_wrap {
                    return Some(quote_spanned! (ident.span() => Some (#value)));
                }

                let unwrapped_value = value.clone().unwrap();
                return Some(quote_spanned! (ident.span() => #unwrapped_value));
            }

            return Some(quote! {});
        }

        None
    })
}

fn sanitize_option_short(
    short: proc_macro2::TokenStream,
    field: &Field,
    existing_shorts: &mut Vec<String>,
    auto_shorts: bool,
) -> proc_macro2::TokenStream {
    // If the TokenStream is empty it means we are dealing with #[kurisu(short)] with no value or no annotation at all
    if short.is_empty() || auto_shorts {
        let short_letter: String = field.ident.clone().unwrap().to_string().drain(0..1).collect();
        if existing_shorts.contains(&short_letter) {
            return syn::Error::new(field.span(), format!("Short flag -{} already bound to another field", short_letter)).to_compile_error();
        }

        existing_shorts.push(short_letter.clone());
        return quote_spanned! (field.ident.span() => Some (#short_letter));
    }

    let mut short_string = short.to_string();
    if short_string.starts_with("Some") {
        let short_letter: String = short_string.drain(6..7).collect();
        if existing_shorts.contains(&short_letter) {
            return syn::Error::new(field.span(), format!("Short flag -{} already bound to another field", short_letter)).to_compile_error();
        }

        existing_shorts.push(short_letter.clone());
        return quote_spanned! (field.ident.span() => Some (#short_letter));
    }

    short
}

fn sanitize_option_long(long: proc_macro2::TokenStream, field: &Field, existing_longs: &mut Vec<String>) -> proc_macro2::TokenStream {
    // If the TokenStream is empty it means we are dealing with #[kurisu(long)] with no value or no annotation at all (default)
    if long.is_empty() {
        let name = field.ident.clone().unwrap();
        let name_string = name.to_string().replace('_', "-");
        if existing_longs.contains(&name_string) {
            return syn::Error::new(field.span(), format!("Long flag --{} already bound to another field", name_string)).to_compile_error();
        }

        existing_longs.push(name_string.clone());
        return quote_spanned! (field.ident.span() => Some (#name_string));
    }

    let long_string = long.to_string();
    if long_string.starts_with("Some") {
        let long: String = long_string.replace("Some(\"", "").replace("Some (\"", "").replace("\")", "");
        if existing_longs.contains(&long) {
            return syn::Error::new(field.span(), format!("Long flag --{} already bound to another field", long)).to_compile_error();
        }

        existing_longs.push(long.clone());
        return quote_spanned! (field.ident.span() => Some (#long));
    }

    long
}

fn impl_kurisu_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let struct_meta_attrs = meta_attributes(&ast.attrs);
    let script_doc = meta_value("doc", &struct_meta_attrs, true).unwrap_or(quote! {None});
    let script_nosort = meta_value("nosort", &struct_meta_attrs, true).is_some();
    let script_noargs = meta_value("allow_noargs", &struct_meta_attrs, true).is_some();
    let auto_shorts = meta_value("auto_shorts", &struct_meta_attrs, true).is_some();
    let mut script_desc = meta_value("desc", &struct_meta_attrs, true).unwrap_or(quote! {None});
    let mut script_version = meta_value("version", &struct_meta_attrs, true).unwrap_or(quote! {None});
    let mut script_name = meta_value("name", &struct_meta_attrs, true).unwrap_or(quote! {None});
    let script_cargo = meta_value("cargo", &struct_meta_attrs, false);
    if script_cargo.is_some() {
        // CARGO_PKG_AUTHORS, CARGO_PKG_HOMEPAGE
        // println!("{:#?}", std::env::vars().collect::<std::collections::HashMap<String, String>>());
        if &script_name.to_string() == "None" {
            let cargo_name = std::env::var("CARGO_PKG_NAME");
            if let Ok(name) = cargo_name {
                script_name = quote_spanned!(script_cargo.span() => Some (#name));
            }
        }

        if &script_version.to_string() == "None" {
            let cargo_version = std::env::var("CARGO_PKG_VERSION");
            if let Ok(version) = cargo_version {
                script_version = quote_spanned!(script_cargo.span() => Some (#version));
            }
        }

        if &script_desc.to_string() == "None" {
            let cargo_description = std::env::var("CARGO_PKG_DESCRIPTION");
            if let Ok(desc) = cargo_description {
                script_desc = quote_spanned!(script_cargo.span() => Some (#desc));
            }
        }
    }

    let mut has_pos_infinite = false;
    let mut existing_shorts: Vec<String> = vec![String::from("V"), String::from("h")];
    let mut existing_longs: Vec<String> = vec![String::from("version"), String::from("help")];
    let fields = get_fields_named(&ast.data);
    let args_array = fields.named.iter().map(|f| {
        let name = &f.ident.clone().unwrap();
        let ty = &f.ty;

        let field_meta_attrs = meta_attributes(&f.attrs);
        let value_name = meta_value("vname", &field_meta_attrs, true).unwrap_or(quote! {None});
        let field_doc = meta_value("doc", &field_meta_attrs, true).unwrap_or(quote! {None});
        let field_default = meta_value("default", &field_meta_attrs, false).unwrap_or(quote! {""});
        let required_if = meta_value("required_if", &field_meta_attrs, true).unwrap_or(quote! {None});
        let env_prefix = meta_value("env_prefix", &field_meta_attrs, true).unwrap_or(quote! {None});
        let env = meta_value("env", &field_meta_attrs, true).unwrap_or(quote! {None});
        let mut field_short = meta_value("short", &field_meta_attrs, true).unwrap_or(quote! {None});
        let mut field_long = meta_value("long", &field_meta_attrs, true).unwrap_or(quote! {});
        let field_input = meta_value("pos", &field_meta_attrs, false);
        let mut input_position = quote! {None};
        if let Some(position) = field_input {
            field_short = quote! {None};
            field_long = quote! {None};

            // If its empty it means we got #[kurisu(pos)] without any value, thus infinite positional arguments
            if position.is_empty() {
                if has_pos_infinite {
                    abort_call_site!("kurisu macro only support one infinite positional argument");
                }

                has_pos_infinite = true;
                input_position = quote! {Some (0)};
            } else {
                input_position = quote_spanned! (position.span() => Some (#position));
            }
        } else {
            field_short = sanitize_option_short(field_short, f, &mut existing_shorts, auto_shorts);

            let nolong = meta_value("nolong", &field_meta_attrs, true).unwrap_or(quote! {None});
            // If nolong is empty it means that the option exists with no value
            if nolong.is_empty() {
                field_long = quote! {None};
            } else {
                field_long = sanitize_option_long(field_long, f, &mut existing_longs);
            }
        }

        let mut exit_cb = quote! {None};
        let field_exit = meta_value("exit", &field_meta_attrs, false);
        if let Some(cb) = field_exit {
            let func_name = cb.to_string();
            let ident = syn::Ident::new(&func_name.trim_matches('"'), cb.span());
            exit_cb = quote! {Some(#ident)}
        }

        let field_aliases = meta_value("aliases", &field_meta_attrs, false).unwrap_or(quote! {});
        let aliases = field_aliases
            .to_string()
            .split(',')
            .map(|a| a.trim())
            .map(|a| a.replace('"', ""))
            .collect::<Vec<String>>();

        quote_spanned! (name.span() => {
            ::kurisu::Arg {
                name: stringify!(#name),
                vname: #value_name,
                value_type: stringify!(#ty),
                position: #input_position,
                short: #field_short,
                long: #field_long,
                aliases: vec![#(#aliases),*],
                doc: #field_doc,
                exit: #exit_cb,
                env: #env,
                env_prefix: #env_prefix,
                required_if: #required_if,
                default: #field_default,
                value: Vec::new(),
                occurrences: 0,
            }
        })
    });

    // TODO: Implement the possibility to skip a struct field by prefixing it with _, Should not generate an Arg and should set default value to struct
    let struct_values = fields.named.iter().map(|f| {
        let name = &f.ident.clone().unwrap();

        let field_meta_attrs = meta_attributes(&f.attrs);
        let field_parser = meta_value("parse_with", &field_meta_attrs, false);
        if let Some(cb) = field_parser {
            let func_name = cb.to_string();
            let ident = syn::Ident::new(&func_name.trim_matches('"'), cb.span());
            quote_spanned! (name.span() => #name: #ident(stringify!(#name), &info),)
        } else {
            quote_spanned! (name.span() => #name: ::kurisu::parse_value(stringify!(#name), &info),)
        }
    });

    let gen = quote! {
        impl ::kurisu::Kurisu for #name {
            fn from_args(env_args: Vec<String>) -> Self {
                let info = Self::get_info_instance(env_args).lock().unwrap();
                ::kurisu::exit_args(&info, |code| { std::process::exit(code); });
                #name {
                    #(#struct_values)*
                }
            }

            fn get_info_instance(env_args: Vec<String>) -> &'static std::sync::Mutex<::kurisu::Info<'static>> {
                static INSTANCE: ::kurisu::OnceCell<std::sync::Mutex<::kurisu::Info>> = ::kurisu::OnceCell::new();
                INSTANCE.get_or_init(move || {
                    let mut kurisu_args = vec![
                        ::kurisu::Arg {
                            name: "usage",
                            vname: None,
                            value_type: "bool",
                            position: None,
                            short: Some("h"),
                            long: Some("help"),
                            aliases: Vec::new(),
                            doc: Some("Prints this message"),
                            exit: None,
                            env: None,
                            env_prefix: None,
                            required_if: None,
                            default: "false",
                            value: Vec::new(),
                            occurrences: 0,
                        },
                        ::kurisu::Arg {
                            name: "version",
                            vname: None,
                            value_type: "bool",
                            position: None,
                            short: Some("V"),
                            long: Some("version"),
                            aliases: Vec::new(),
                            doc: Some("Prints version information"),
                            exit: None,
                            env: None,
                            env_prefix: None,
                            required_if: None,
                            default: "false",
                            value: Vec::new(),
                            occurrences: 0,
                        },
                        #(#args_array),*
                    ];

                    let mut env_args = ::kurisu::normalize_env_args(&env_args, &kurisu_args);
                    let positions: Vec<i8> = kurisu_args
                        .iter()
                        .filter(|a| a.position.is_some() && a.position.unwrap() != 0)
                        .map(|a| a.position.unwrap())
                        .collect();

                    for arg in kurisu_args.iter_mut() {
                        arg.set_value(&mut env_args, &positions);
                    }

                    // Sort args for Display usage
                    if !#script_nosort {
                        kurisu_args.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    }

                    std::sync::Mutex::new(Info {
                        name: #script_name,
                        version: #script_version,
                        desc: #script_desc,
                        doc: #script_doc,
                        allow_noargs: #script_noargs,
                        env_args,
                        args: kurisu_args,
                    })
                })
            }
        }
    };
    gen.into()
}
