use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_str,
    punctuated::Punctuated,
    FieldValue, Token, Type,
};

struct Components {
    first: Ident,
    rest: Vec<Ident>,
    defaults: Vec<FieldValue>,
}

impl Parse for Components {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first: Ident = input.parse()?;
        let mut rest: Vec<Ident> = vec![first.clone()];

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            // Если следующий токен — `default`, прерываем
            if input.peek(syn::Ident) && input.peek2(Token![:]) {
                let lookahead = input.fork();
                let ident: Ident = lookahead.parse()?;
                if ident == "default" {
                    break;
                }
            }
            rest.push(input.parse()?);
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        let mut defaults = Vec::new();
        if input.peek(syn::Ident) {
            let kw: Ident = input.parse()?;
            if kw == "default" {
                input.parse::<Token![:]>()?;
                let content;
                braced!(content in input);

                let parsed: Punctuated<FieldValue, Token![,]> =
                    content.parse_terminated(FieldValue::parse, syn::Token![,])?;
                defaults.extend(parsed);
            }
        }

        Ok(Components {
            first,
            rest,
            defaults,
        })
    }
}

impl Components {
    fn generate_default(&self) -> Vec<syn::ExprAssign> {
        for default in &self.defaults {
            let default_str = default.into_token_stream().to_string();
            if !default_str.starts_with("desired_size") {
                continue;
            }

            let (left, right) = default_str.split_once(':').unwrap();
            let result = format!("widget_base.{left} = {right}");

            let expr_assign = syn::parse_str::<syn::ExprAssign>(&result)
                .unwrap_or_else(|_| panic!("Failed to parse: {result}"));

            return vec![expr_assign];
        }

        vec![]
    }
}

fn to_snake_case(ident: &Ident) -> Ident {
    let mut s = String::new();
    let name = ident.to_string();

    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                s.push('_');
            }
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c);
        }
    }

    Ident::new(&s, Span::call_site())
}

const ENTITY: &str = "bevy_ecs::prelude::Entity";
const COMMANDS: &str = "bevy_ecs::prelude::Commands<'world, 'state>";
const CHILD_OF: &str = "bevy_ecs::prelude::ChildOf";
const BUNDLE: &str = "bevy_ecs::prelude::Bundle";
const DESIRED_SIZE: &str = "toolkit::widget::DesiredSize";

#[proc_macro]
pub fn define_widget(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let entity_ty: Type = parse_str(ENTITY).unwrap();
    let commands_ty: Type = parse_str(COMMANDS).unwrap();
    let child_of_ty: Type = parse_str(CHILD_OF).unwrap();
    let bundle_ty: Type = parse_str(BUNDLE).unwrap();
    let desired_size_ty: Type = parse_str(DESIRED_SIZE).unwrap();

    let input = proc_macro2::TokenStream::from(input);
    let components: Components = syn::parse2(input).expect("dfsfs");

    let field_defaults = components.generate_default();
    //panic!("{:#?}", field_defaults);
    let mut fields = Vec::new();
    let mut defaults = Vec::new();
    let mut setters = Vec::new();

    let first = &components.first;
    let components = &components.rest;

    components.iter().for_each(|component| {
        let field_name = to_snake_case(component);
        fields.push(quote! { pub #field_name: #component, });
        defaults.push(quote! { #field_name: Default::default(), });
        if component == "Color" {
            setters.push(quote! {
                pub fn color(mut self, value: impl Into<Color>) -> Self {
                    self.bundle.color = value.into();
                    self
                }
            });
        } else {
            setters.push(quote! {
                pub fn #field_name(mut self, value: #component) -> Self {
                    self.bundle.#field_name = value;
                    self
                }
            });
        }
    });

    let bundle_name = Ident::new(&format!("{first}Bundle"), Span::call_site());
    let builder_name = Ident::new(&format!("{first}Builder"), Span::call_site());
    quote! {
        #[derive(#bundle_ty)]
        pub struct #bundle_name {
            widget_base: toolkit::WidgetBundle,
            #(#fields)*
        }

        impl Default for #bundle_name {
            fn default() -> Self {
                let mut widget_base = toolkit::WidgetBundle::default();
                #(#field_defaults;)*
                Self {
                    widget_base,
                    #(#defaults)*
                }
            }
        }

        pub struct #builder_name<'commands, 'world, 'state> {
            commands: &'commands mut #commands_ty,
            bundle: #bundle_name
        }

        impl<'commands, 'world, 'state> #builder_name<'commands, 'world, 'state> {
            pub fn new(commands: &'commands mut #commands_ty) -> Self {
                Self {
                    commands,
                    bundle: #bundle_name::default()
                }
            }

            #(#setters)*

            pub fn desired_size(mut self, value: #desired_size_ty) -> Self {
                self.bundle.widget_base.desired_size = value;
                self
            }

            pub fn build_as_child_of(self, parent: #entity_ty) -> #entity_ty {
                self.commands.spawn((self.bundle, #child_of_ty(parent))).id()
            }

            pub fn build(self) -> #entity_ty {
                self.commands.spawn(self.bundle).id()
            }
        }
    }
    .into()
}
