extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(WidgetEnum)]
pub fn widget_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants.into_iter().map(|v| {
            let vname = v.ident;
            match v.fields {
                Fields::Unnamed(fields) => {
                    let inner = &fields.unnamed[0].ty;
                    (vname, inner.clone())
                },
                _ => panic!("WidgetEnum only supports tuple enums with one field")
            }
        }).collect::<Vec<_>>()
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
        quote! { #name::#vname(inner) => inner.update(ctx), }
    });

    let expanded = quote! {
        impl toolkit::widget::Widget for #name {
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

            fn update(&mut self, ctx: &toolkit::widget::FrameContext) {
                match self {
                    #(#update_match)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(WindowRootEnum, attributes(window_gui))]
pub fn derive_window_root_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;
    let gui_type = input.attrs.iter()
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
        quote! {
            #enum_name::#vname(inner) => inner.update(gui, ctx)
        }
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
                match self {
                    #(#arms_update),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
