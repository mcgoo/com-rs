use proc_macro2::TokenStream as HelperTokenStream;
use quote::quote;
use syn::{Ident, ItemStruct, Fields};
use std::collections::HashMap;

/// The actual COM object that wraps around the Init struct.
/// Structure of the object:
/// pub struct _ {
///     ..base interface vpointers..
///     ..ref count..
///     ..init struct..
/// }
pub fn generate(aggr_map: &HashMap<Ident, Vec<Ident>>, base_itf_idents: &[Ident], struct_item: &ItemStruct) -> HelperTokenStream {
    let struct_ident = &struct_item.ident;
    let vis = &struct_item.vis;

    let bases_itf_idents = base_itf_idents.iter().map(|base| {
        let field_ident = macro_utils::get_vptr_field_ident(&base);
        quote!(#field_ident: <dyn #base as com::ComInterface>::VPtr)
    });

    let ref_count_ident = macro_utils::get_ref_count_ident();

    let fields = match &struct_item.fields {
        Fields::Named(f) => &f.named,
        _ => panic!("Found non Named fields in struct.")
    };

    let aggregates = aggr_map.iter().map(|(aggr_field_ident, aggr_base_itf_idents)| {
        quote!(
            #aggr_field_ident: *mut <dyn com::IUnknown as com::ComInterface>::VPtr
        )
    });

    quote!(
        #[repr(C)]
        #vis struct #struct_ident {
            #(#bases_itf_idents,)*
            #ref_count_ident: u32,
            #(#aggregates,)*
            #fields
        }
    )
}
