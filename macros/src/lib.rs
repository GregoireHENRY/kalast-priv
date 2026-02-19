use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, Path};
use std::fs;
use std::path::PathBuf;

#[proc_macro_attribute]
pub fn pybind(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    
    // Find the field marked with #[pybind]
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("pybind only works on structs with named fields"),
        },
        _ => panic!("pybind only works on structs"),
    };
    
    let mut target_path = None;
    for field in fields {
        // Look for #[pybind] attribute
        for attr in &field.attrs {
            if attr.path().is_ident("pybind") {
                target_path = Some(extract_inner_type(&field.ty));
                /* if need nested args
                if let Meta::List(meta_list) = &attr.meta {
                    let nested = &meta_list.tokens.to_string();
                    if nested == "vec" {
                        target_path = Some(extract_inner_type(&field.ty));
                        break;
                    }
                }
                */
            }
        }
        
        if target_path.is_some() {
            break;
        }
    }
    
    let target_path = target_path.expect("No field marked with #[pybind]");
    
    // Parse the target struct to find Vec2/Vec3 fields
    let (vec2_fields, vec3_fields) = find_vec_fields(&target_path);
    
    // Generate methods
    let methods = generate_vec_methods(&vec2_fields, &vec3_fields);
    
    // Remove #[pybind(vec)] attributes from fields to avoid compiler errors
    if let Data::Struct(ref mut data) = input.data {
        if let Fields::Named(ref mut fields) = data.fields {
            for field in &mut fields.named {
                field.attrs.retain(|attr| !attr.path().is_ident("pybind"));
            }
        }
    }
    
    let expanded = quote! {
        #input
        
        #[pymethods]
        impl #struct_name {
            #methods
        }
    };
    
    TokenStream::from(expanded)
}

fn extract_inner_type(ty: &Type) -> Path {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Rc" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(Type::Path(inner))) = args.args.first() {
                        if let Some(refcell_segment) = inner.path.segments.last() {
                            if refcell_segment.ident == "RefCell" {
                                if let syn::PathArguments::AngleBracketed(refcell_args) = &refcell_segment.arguments {
                                    if let Some(syn::GenericArgument::Type(Type::Path(innermost))) = refcell_args.args.first() {
                                        return innermost.path.clone();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Expected Rc<RefCell<T>> type");
}

fn find_vec_fields(target_path: &Path) -> (Vec<String>, Vec<String>) {
    let module_path = path_to_file_path(target_path);
    
    let content = fs::read_to_string(&module_path)
        .expect(&format!("Failed to read file: {:?}", module_path));
    
    let syntax_tree: syn::File = syn::parse_file(&content)
        .expect("Failed to parse file");
    
    let struct_name = target_path.segments.last().unwrap().ident.to_string();
    
    let mut vec2_fields = Vec::new();
    let mut vec3_fields = Vec::new();
    
    for item in syntax_tree.items {
        if let syn::Item::Struct(item_struct) = item {
            if item_struct.ident == struct_name {
                if let Fields::Named(fields) = item_struct.fields {
                    for field in fields.named {
                        let field_name = field.ident.unwrap().to_string();
                        
                        if is_vec_type(&field.ty, "Vec2") {
                            vec2_fields.push(field_name);
                        } else if is_vec_type(&field.ty, "Vec3") {
                            vec3_fields.push(field_name);
                        }
                    }
                }
                break;
            }
        }
    }
    
    (vec2_fields, vec3_fields)
}

fn path_to_file_path(path: &Path) -> PathBuf {
    let mut file_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    file_path.push("src");
    
    for segment in &path.segments {
        let ident = segment.ident.to_string();
        if ident == "crate" {
            continue;
        }
        file_path.push(&ident);
    }
    
    let mut rs_parent = file_path.parent().unwrap().to_path_buf();
    rs_parent.set_extension("rs");
    
    if rs_parent.exists() {
        rs_parent
    }
    else{
        let mut rs_path = file_path.clone();
        rs_path.set_extension("rs");
        
        if rs_path.exists() {
            rs_path
        } else {
            file_path.push("mod.rs");
            file_path
        }
    }
    
}

fn is_vec_type(ty: &Type, vec_name: &str) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == vec_name;
        }
    }
    false
}

fn generate_vec_methods(vec2_fields: &[String], vec3_fields: &[String]) -> proc_macro2::TokenStream {
    let mut methods = Vec::new();
    
    for field in vec3_fields {
        let field_ident = syn::Ident::new(field, proc_macro2::Span::call_site());
        let setter_ident = syn::Ident::new(&format!("set_{}", field), proc_macro2::Span::call_site());
        
        methods.push(quote! {
            #[getter]
            fn #field_ident<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
                let inner = &slf.borrow().inner;
                let slice = &inner.borrow().#field_ident;
                let arr = ndarray::ArrayView1::from(slice.as_ref());
                unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
            }
            
            #[setter]
            fn #setter_ident(&self, arr: [Float; 3]) {
                self.inner.borrow_mut().#field_ident = arr.into();
            }
        });
    }
    
    for field in vec2_fields {
        let field_ident = syn::Ident::new(field, proc_macro2::Span::call_site());
        let setter_ident = syn::Ident::new(&format!("set_{}", field), proc_macro2::Span::call_site());
        
        methods.push(quote! {
            #[getter]
            fn #field_ident<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
                let inner = &slf.borrow().inner;
                let slice = &inner.borrow().#field_ident;
                let arr = ndarray::ArrayView1::from(slice.as_ref());
                unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
            }
            
            #[setter]
            fn #setter_ident(&self, arr: [Float; 2]) {
                self.inner.borrow_mut().#field_ident = arr.into();
            }
        });
    }
    
    quote! {
        #(#methods)*
    }
}