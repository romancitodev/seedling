use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, Ident, Lit, Token, bracketed, parenthesized, parse::Parse, parse_macro_input,
    punctuated::Punctuated,
};

struct TupleParser {
    key: Ident,
    value: Expr,
}

struct Input {
    schema: Lit,
    name: Ident,
    repetition: Lit,
    fields: Punctuated<TupleParser, Token![,]>,
}

impl Parse for TupleParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let key: Ident = content.parse()?;
        let _: Token![,] = content.parse()?;
        let value: Expr = content.parse()?;

        Ok(Self { key, value })
    }
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let schema: Lit = input.parse()?;
        let _: Token![,] = input.parse()?;

        let name: Ident = input.parse()?;
        let _: Token![,] = input.parse()?;

        let repetition: Lit = input.parse()?;
        let _: Token![,] = input.parse()?;

        let content;
        bracketed!(content in input);
        let fields: Punctuated<TupleParser, Token![,]> =
            content.parse_terminated(TupleParser::parse, Token![,])?;

        Ok(Self {
            schema,
            name,
            repetition,
            fields,
        })
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn create_enum(
    name: &str,
    fields: Punctuated<TupleParser, Token![,]>,
) -> (Ident, proc_macro2::TokenStream) {
    let enum_id = format_ident!("{}Columns", capitalize(name));
    let enum_names = fields.iter().map(|f| f.key.to_string()).collect::<Vec<_>>();
    let enum_fields = fields
        .iter()
        .map(|f| format_ident!("{}", capitalize(&f.key.to_string())))
        .collect::<Vec<_>>();
    let enum_exprs = fields.iter().map(|f| f.value.clone()).collect::<Vec<_>>();

    (
        enum_id.clone(),
        quote! {
            #[derive(Debug)]
            enum #enum_id {
                #(#enum_fields),*
            }

            impl seedling::Column for #enum_id {
                fn all() -> &'static [Self] {
                    &[#(Self::#enum_fields),*]
                }

                fn name(&self) -> &'static str {
                    match self {
                        #(Self::#enum_fields => #enum_names),*
                    }
                }

                fn value(&self) -> impl IntoValue {
                    match &self {
                        #(Self::#enum_fields => #enum_exprs),*
                    }
                }
            }
        }
        .into(),
    )
}

#[proc_macro]
pub fn procedural_generate(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token as Input);

    let Lit::Str(schema) = input.schema else {
        return quote! {}.into();
    };

    let schema = Some(schema.value()).filter(|s| !s.is_empty());

    let Lit::Int(repetition) = input.repetition else {
        return quote! {}.into();
    };

    let schema_id = if let Some(named) = schema.clone() {
        let named = format_ident!("{}Schema", capitalize(&named));
        quote! { #named }
    } else {
        quote! { () }
    };

    let schema = if let Some(_) = schema {
        let named = schema.unwrap();
        quote! {
        struct #schema_id;
        impl seedling::Schema for #schema_id {
            fn schema_name() -> Option<&'static str> {
                Some(#named)
            }
        }
         }
    } else {
        quote! {}
    };

    let table = input.name; // Users
    let table_name = table.to_string();
    let table_id = format_ident!("{}", capitalize(&table.to_string()));
    let users_expand = quote! {
        struct #table_id;
    };

    let (columns, columns_impl) = create_enum(&table_name, input.fields);

    let table_impl = quote! {
        impl seedling::Table<#schema_id> for #table_id {
            type Columns = #columns;

            fn table_name() -> &'static str {
                #table_name
            }
        }
    };

    let output = quote! {
    {
        use seedling::IntoValue;
        #schema
        #users_expand
        #columns_impl
        #table_impl
        seedling::Mock::<#table_id, #schema_id, #repetition>::new()
    }};
    TokenStream::from(output)
}
