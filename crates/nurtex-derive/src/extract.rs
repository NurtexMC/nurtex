use syn::{GenericArgument, PathArguments, Type};

/// Функция извлечения атрибута пакета
pub fn extract_packet_attr(f: &syn::Field) -> Option<String> {
  f.attrs
    .iter()
    .find(|a| a.path().is_ident("packet"))
    .and_then(|a| a.parse_args::<syn::Ident>().ok())
    .map(|i| i.to_string())
}

/// Функция извлечения ID пакета
pub fn extract_packet_id(variant: &syn::Variant) -> Option<u32> {
  variant.attrs.iter().find(|a| a.path().is_ident("packet_id")).and_then(|a| {
    if let syn::Meta::NameValue(nv) = &a.meta {
      if let syn::Expr::Lit(expr_lit) = &nv.value {
        match &expr_lit.lit {
          syn::Lit::Int(lit_int) => {
            let s = lit_int.to_string();
            if s.starts_with("0x") {
              u32::from_str_radix(&s[2..], 16).ok()
            } else {
              lit_int.base10_parse::<u32>().ok()
            }
          }
          syn::Lit::Str(lit_str) => {
            let s = lit_str.value();
            if s.starts_with("0x") {
              u32::from_str_radix(&s[2..], 16).ok()
            } else {
              s.parse::<u32>().ok()
            }
          }
          _ => None,
        }
      } else {
        None
      }
    } else {
      None
    }
  })
}

/// Функция извлечения типа из `Option<T>`
pub fn extract_option_inner_type(ty: &Type) -> Option<Type> {
  if let Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      if segment.ident == "Option" {
        if let PathArguments::AngleBracketed(args) = &segment.arguments {
          if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
            return Some(inner_ty.clone());
          }
        }
      }
    }
  }

  None
}
