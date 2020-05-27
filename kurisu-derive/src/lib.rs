extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
// TODO: Apply Macro hygiene as much as possible with quote_spanned
use quote::quote;
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
    let mut doc_lines: Vec<String> = vec![];
    let mut attributes: Vec<(proc_macro2::Ident, Option<syn::Lit>)> = vec![];
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
                doc_lines.join(" ").trim(),
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
                    return Some(quote! { Some (#value) });
                }

                let unwrapped_value = value.clone().unwrap();
                return Some(quote! { #unwrapped_value });
            }

            return Some(quote! {});
        }

        None
    })
}

fn sanitize_option_short(short: proc_macro2::TokenStream, field: &Field, existing_shorts: &mut Vec<String>) -> proc_macro2::TokenStream {
    // If the TokenStream is empty it means we are dealing with #[kurisu(short)] with no value or no annotation at all
    if short.is_empty() {
        let short_letter: String = field.ident.clone().unwrap().to_string().drain(0..1).collect();
        if existing_shorts.contains(&short_letter) {
            return syn::Error::new(field.span(), format!("Short flag -{} already bound to another field", short_letter)).to_compile_error();
        }

        existing_shorts.push(short_letter.clone());
        return quote! { Some (#short_letter) };
    }

    let mut short_string = short.to_string();
    if short_string.starts_with("Some") {
        let short_letter: String = short_string.drain(7..8).collect();
        if existing_shorts.contains(&short_letter) {
            return syn::Error::new(field.span(), format!("Short flag -{} already bound to another field", short_letter)).to_compile_error();
        }

        existing_shorts.push(short_letter.clone());
        return quote! {Some (#short_letter)};
    }

    short
}

fn sanitize_option_long(long: proc_macro2::TokenStream, field: &Field, existing_longs: &mut Vec<String>) -> proc_macro2::TokenStream {
    // TODO: Rename feature, like camelCase?
    // If the TokenStream is empty it means we are dealing with #[kurisu(long)] with no value or no annotation at all (default)
    if long.is_empty() {
        let name = field.ident.clone().unwrap();
        let name_string = name.to_string().replace('_', "-");
        if existing_longs.contains(&name_string) {
            return syn::Error::new(field.span(), format!("Long flag --{} already bound to another field", name_string)).to_compile_error();
        }

        existing_longs.push(name_string.clone());
        return quote! { Some (#name_string) };
    }

    let long_string = long.to_string();
    if long_string.starts_with("Some") {
        let long: String = long_string.replace("Some(\"", "").replace("Some (\"", "").replace("\")", "");
        if existing_longs.contains(&long) {
            return syn::Error::new(field.span(), format!("Long flag --{} already bound to another field", long)).to_compile_error();
        }

        existing_longs.push(long.clone());
        return quote! {Some (#long)};
    }

    long
}

fn impl_kurisu_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let struct_meta_attrs = meta_attributes(&ast.attrs);
    let mut script_doc = meta_value("doc", &struct_meta_attrs, true).unwrap_or(quote! {None});
    let mut script_version = meta_value("version", &struct_meta_attrs, true).unwrap_or(quote! {None});
    let mut script_name = meta_value("name", &struct_meta_attrs, true).unwrap_or(quote! {None});
    let script_cargo = meta_value("cargo", &struct_meta_attrs, false);
    if script_cargo.is_some() {
        // CARGO_PKG_AUTHORS, CARGO_PKG_HOMEPAGE
        // println!("{:#?}", std::env::vars().collect::<std::collections::HashMap<String, String>>());
        let cargo_name = std::env::var("CARGO_PKG_NAME");
        if let Ok(name) = cargo_name {
            script_name = quote! { Some (#name) };
        }

        let cargo_version = std::env::var("CARGO_PKG_VERSION");
        if let Ok(version) = cargo_version {
            script_version = quote! { Some (#version) };
        }

        let cargo_description = std::env::var("CARGO_PKG_DESCRIPTION");
        if let Ok(doc) = cargo_description {
            script_doc = quote! { Some (#doc) };
        }
    }

    let mut script_noargs = meta_value("noargs", &struct_meta_attrs, false).unwrap_or(quote! {false});
    // If noargs is empty it means that the option exists with no value
    if script_noargs.is_empty() {
        script_noargs = quote! {true};
    }

    let mut existing_shorts: Vec<String> = vec![String::from("v"), String::from("h")];
    let mut existing_longs: Vec<String> = vec![String::from("version"), String::from("help")];
    let fields = get_fields_named(&ast.data);
    let args_array = fields.named.iter().map(|f| {
        let name = &f.ident.clone().unwrap();
        let ty = &f.ty;

        let field_meta_attrs = meta_attributes(&f.attrs);
        let field_doc = meta_value("doc", &field_meta_attrs, true).unwrap_or(quote! {None});
        let field_default = meta_value("default", &field_meta_attrs, false).unwrap_or(quote! {""});
        let mut field_short = meta_value("short", &field_meta_attrs, true).unwrap_or(quote! {None});
        let mut field_long = meta_value("long", &field_meta_attrs, true).unwrap_or(quote! {});
        let field_input = meta_value("pos", &field_meta_attrs, false);
        let mut input_position = quote! {None};
        if let Some(position) = field_input {
            field_short = quote! {None};
            field_long = quote! {None};

            // If its empty it means we got #[kurisu(pos)] without any value, thus infinite positional arguments
            if position.is_empty() {
                input_position = quote! {Some (0)};
            } else {
                input_position = quote! {Some (#position)};
            }
        } else {
            field_short = sanitize_option_short(field_short, f, &mut existing_shorts);

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

        quote! {
            ::kurisu::Arg {
                name: stringify!(#name),
                value_type: stringify!(#ty),
                position: #input_position,
                short: #field_short,
                long: #field_long,
                doc: #field_doc,
                exit: #exit_cb,
                default: #field_default,
                value: Vec::new(),
                occurrences: 0,
            }
        }
    });

    let struct_values = fields.named.iter().map(|f| {
        let name = &f.ident.clone().unwrap();
        quote! { #name: ::kurisu::parse_value(stringify!(#name), &info), }
    });

    let gen = quote! {
        impl<'a> ::kurisu::Kurisu<'a> for #name {
            fn from_args(env_args: Vec<String>) -> #name {
                let mutex = Self::get_info_instance(env_args);
                let info = mutex.lock().unwrap();
                ::kurisu::exit_args(&info);
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
                            value_type: "bool",
                            position: None,
                            short: Some("h"),
                            long: Some("help"),
                            doc: Some("Prints this message"),
                            exit: None,
                            default: "false",
                            value: Vec::new(),
                            occurrences: 0,
                        },
                        ::kurisu::Arg {
                            name: "version",
                            value_type: "bool",
                            position: None,
                            short: None,
                            long: Some("version"),
                            doc: Some("Prints version information"),
                            exit: None,
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

                    std::sync::Mutex::new(Info {
                        name: #script_name,
                        version: #script_version,
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
