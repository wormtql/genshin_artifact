use syn::DeriveInput;

pub fn get_enum_variants(ast: &DeriveInput) -> Vec<String> {
    let mut temp = Vec::new();
    match ast.data {
        syn::Data::Enum(ref e) => {
            for v in e.variants.iter() {
                temp.push(v.ident.to_string());
            }
        },
        _ => panic!("only works for enum")
    }

    temp
}
