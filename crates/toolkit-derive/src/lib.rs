#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Attribute, Data, DeriveInput, Fields, Meta};

fn extract_context_attr(attrs: &[Attribute]) -> Result<Option<syn::Type>, syn::Error> {
    for attr in attrs {
        if attr.path().is_ident("context") {
            return match &attr.meta {
                Meta::List(meta_list) => {
                    // Парсим содержимое внутри #[context(...)]
                    let parser = syn::parse::Parser::parse2;
                    let context_type: syn::Type =
                        parser(syn::Type::parse, meta_list.tokens.clone())?;
                    Ok(Some(context_type))
                }
                Meta::NameValue(name_value) => {
                    // Парсим #[context = "TypeName"]
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit_str),
                        ..
                    }) = &name_value.value
                    {
                        let type_str = lit_str.value();
                        let context_type: syn::Type = syn::parse_str(&type_str)?;
                        Ok(Some(context_type))
                    } else {
                        Err(syn::Error::new_spanned(
                            &name_value.value,
                            "Expected string literal, e.g. #[context = \"MyAppEvent\"]",
                        ))
                    }
                }
                Meta::Path(_) => Err(syn::Error::new_spanned(
                    attr,
                    "Expected context type. Use #[context(TypeName)] or #[context = \"TypeName\"]",
                )),
            };
        }
    }
    Ok(None)
}

#[proc_macro_derive(WidgetEnum, attributes(context))]
pub fn widget_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Извлекаем атрибут #[context]
    let context_type = extract_context_attr(&input.attrs).expect("Missing attribute #[context]");

    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum
            .variants
            .into_iter()
            .map(|v| {
                let vname = v.ident;
                match v.fields {
                    Fields::Unnamed(fields) => {
                        let inner = &fields.unnamed[0].ty;
                        (vname, inner.clone())
                    }
                    _ => panic!("WidgetEnum only supports tuple enums with one field"),
                }
            })
            .collect::<Vec<_>>()
    } else {
        panic!("WidgetEnum can only be derived for enums");
    };

    let id_match = variants.iter().map(|(vname, _)| {
        quote! { #name::#vname(inner) => inner.id(), }
    });

    let desired_size_match = variants.iter().map(|(vname, _)| {
        quote! { #name::#vname(inner) => inner.desired_size(), }
    });

    let as_any_match = variants.iter().map(|(vname, _)| {
        quote! { #name::#vname(inner) => inner, }
    });

    let as_any_mut_match = as_any_match.clone();

    let draw_match = variants.iter().map(|(vname, _)| {
        quote! { #name::#vname(inner) => inner.draw(out), }
    });

    let layout_match = variants.iter().map(|(vname, _)| {
        quote! { #name::#vname(inner) => inner.layout(bounds), }
    });

    let update_match = variants.iter().map(|(vname, _)| {
        quote! { #name::#vname(inner) => inner.update(ctx, sender), }
    });

    let expanded = quote! {
        impl toolkit::widget::Widget<#context_type> for #name {
            fn id(&self) -> Option<&str> {
                match self {
                    #(#id_match)*
                }
            }

            fn desired_size(&self) -> toolkit::widget::DesiredSize {
                match self {
                    #(#desired_size_match)*
                }
            }

            fn as_any(&self) -> &dyn std::any::Any {
                match self {
                    #(#as_any_match)*
                }
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                match self {
                    #(#as_any_mut_match)*
                }
            }

            fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
                match self {
                    #(#draw_match)*
                }
            }

            fn layout(&mut self, bounds: toolkit::types::Rect) {
                match self {
                    #(#layout_match)*
                }
            }

            fn update(&mut self, ctx: &toolkit::widget::FrameContext, sender: &mut toolkit::widget::Sender<#context_type>) {
                match self {
                    #(#update_match)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(WindowRootEnum, attributes(window_gui, context))]
pub fn derive_window_root_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;
    let gui_type = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("window_gui"))
        .and_then(|a| a.parse_args::<syn::Type>().ok())
        .expect("You must add #[window_gui(AppType)] attribute to the enum");

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("#[derive(WindowRootEnum)] can only be used on enums"),
    };

    let arms_request = variants.iter().map(|v| {
        let vname = &v.ident;
        let inner = match &v.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => quote! { inner },
            _ => panic!("Each variant must be a tuple with exactly 1 field"),
        };
        quote! {
            #enum_name::#vname(#inner) => #inner.request()
        }
    });

    let arms_setup = variants.iter().map(|v| {
        let vname = &v.ident;
        quote! {
            #enum_name::#vname(inner) => inner.setup(gui)
        }
    });

    let arms_draw = variants.iter().map(|v| {
        let vname = &v.ident;
        quote! {
            #enum_name::#vname(inner) => inner.draw(out)
        }
    });

    let arms_layout = variants.iter().map(|v| {
        let vname = &v.ident;
        quote! {
            #enum_name::#vname(inner) => inner.layout(bounds)
        }
    });

    let arms_update = variants.iter().map(|v| {
        let vname = &v.ident;
        //quote! {
        //    #enum_name::#vname(inner) => inner.update(gui, ctx)
        //}
    });

    let expanded = quote! {
        impl WindowRoot for #enum_name {
            type Gui = #gui_type;

            fn request(&self) -> WindowRequest {
                match self {
                    #(#arms_request),*
                }
            }

            fn setup(&mut self, gui: &mut #gui_type) {
                match self {
                    #(#arms_setup),*
                }
            }

            fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
                match self {
                    #(#arms_draw),*
                }
            }

            fn layout(&mut self, bounds: toolkit::types::Rect) {
                match self {
                    #(#arms_layout),*
                }
            }

            fn update(&mut self, gui: &mut #gui_type, ctx: &toolkit::widget::FrameContext) {
                //match self {
                //    #(#arms_update),*
                //}
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(WidgetQuery, attributes(content))]
pub fn widget_query_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    // Извлекаем информацию о полях
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("WidgetQuery can only be derived for structs with named fields"),
        },
        _ => panic!("WidgetQuery can only be derived for structs"),
    };

    // Находим поле с атрибутом #[content]
    let content_field = fields.iter().find(|field| {
        field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("content"))
    });

    // Генерируем реализацию в зависимости от наличия content поля
    let impl_block = if let Some(content_field) = content_field {
        let content_field_name = content_field.ident.as_ref();
        generate_with_content(name, generics, content_field_name)
    } else {
        generate_without_content(name, generics)
    };

    TokenStream::from(impl_block)
}

fn generate_with_content(
    name: &syn::Ident,
    generics: &syn::Generics,
    content_field_name: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let content_field_name = content_field_name
        .as_ref()
        .expect("Content field must have a name");

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics toolkit::widget::WidgetQuery<C> for #name #ty_generics #where_clause {
            fn get_element<QW: Widget<C>>(&self, id: &str) -> Option<&QW> {
                // Сначала проверяем собственный ID
                if let Some(ref self_id) = self.id {
                    if self_id == id {
                        return self.as_any().downcast_ref::<QW>();
                    }
                }

                // Затем делегируем к content полю
                self.#content_field_name.get_element(id)
            }

            fn get_mut_element<QW: Widget<C>>(&mut self, id: &str) -> Option<&mut QW> {
                // Сначала проверяем собственный ID
                if let Some(ref self_id) = self.id {
                    if self_id == id {
                        return self.as_any_mut().downcast_mut::<QW>();
                    }
                }

                // Затем делегируем к content полю
                self.#content_field_name.get_mut_element(id)
            }
        }
    }
}

fn generate_without_content(
    name: &syn::Ident,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics toolkit::widget::WidgetQuery<C> for #name #ty_generics #where_clause {
            fn get_element<QW: Widget<C>>(&self, id: &str) -> Option<&QW> {
                if let Some(ref self_id) = self.id {
                    if self_id == id {
                        return self.as_any().downcast_ref::<QW>();
                    }
                }
                None
            }

            fn get_mut_element<QW: Widget<C>>(&mut self, id: &str) -> Option<&mut QW> {
                if let Some(ref self_id) = self.id {
                    if self_id == id {
                        return self.as_any_mut().downcast_mut::<QW>();
                    }
                }
                None
            }
        }
    }
}
