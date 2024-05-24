use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod ser;
mod de;

use ser::proto_build_ser;
use de::proto_build_de;

#[proc_macro_derive(ProtoCodec)]
pub fn proto_serialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;

    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ser = proto_build_ser(&input.data);
    let de = proto_build_de(&input.data);

    let expanded = quote! {
        impl #impl_generics proto_core::ProtoCodec for #name #ty_generics #where_clause {
            fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), proto_core::error::ProtoCodecError> where Self: Sized {
                #ser
                Ok(())
            }

            fn proto_deserialize(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, proto_core::error::ProtoCodecError> where Self: Sized {
                Ok(Self{
                    #de
                })
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
