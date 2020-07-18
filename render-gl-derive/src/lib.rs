use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_vertex_attrib_pointers(&ast).into()
}

fn impl_vertex_attrib_pointers(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;

    let calls = generate_vertex_attrib_pointer_calls(&ast.data);

    let gen = quote! {
        impl #generics #ident #generics #where_clause {
            #[allow(unused_variables)]
            pub fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = ::std::mem::size_of::<Self>();
                let offset = 0;

                #(#calls)*
            }
        }
    };
    gen
}

fn generate_vertex_attrib_pointer_calls(data: &syn::Data) -> Vec<TokenStream> {
    let fields = match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    fields.iter().map(generate_struct_field_vertex_attrib_pointer_call).collect()
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> TokenStream {
    let field_name = match field.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };
    let location_attr = field
        .attrs
        .iter()
        .filter(|a| a.path.is_ident("location"))
        .next()
        .unwrap_or_else(|| panic!("Field {} is missing #[location = ?] attribute", field_name));

    let location_meta = match location_attr.parse_meta().unwrap_or_else(|_| {
        panic!(
            "unable to parse meta for location attr in field {}",
            field_name
        )
    }) {
        syn::Meta::NameValue(val) => val,
        _ => panic!("Location value must be a name value meta in field {}", field_name),
    };

    let location_value = match location_meta.lit {
        syn::Lit::Int(digit) => digit.base10_parse::<usize>().unwrap(),
        _ => panic!("invalid data type for location value in field {}!", field_name),
    };

    let field_ty = &field.ty;

    let call = quote! {
        let location = #location_value;
        unsafe {
            #field_ty::vertex_attrib_pointer(gl, stride, location, offset);
        }
        let offset = offset + ::std::mem::size_of::<#field_ty>();
    };

    call
}
