//! Documentation generator for Innoframework source code.
//!
//! Parses Rust source files using `syn` and generates markdown documentation.
//! Extracts public items (structs, enums, functions, traits, impl blocks)
//! along with their doc comments.

use quote::quote;
use std::path::Path;
use walkdir::WalkDir;

/// A documentation item extracted from source.
#[derive(Debug, Clone)]
pub struct DocItem {
    pub kind: DocKind,
    pub name: String,
    pub doc: String,
    pub signature: String,
    pub location: String,
    pub children: Vec<DocItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocKind {
    Struct,
    Enum,
    Function,
    Trait,
    Impl,
    Module,
    Constant,
}

/// Generated documentation for a crate/module.
#[derive(Debug, Clone)]
pub struct DocOutput {
    pub module_name: String,
    pub items: Vec<DocItem>,
}

/// Parse a Rust source file into documentation items.
pub fn parse_file(path: &Path) -> Result<Vec<DocItem>, Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;
    let file: syn::File = syn::parse_file(&source)?;
    let mut items = Vec::new();

    for item in &file.items {
        if let Some(doc) = extract_item(item, path)? {
            items.push(doc);
        }
    }

    Ok(items)
}

/// Extract a documentation item from a syn item.
fn extract_item(item: &syn::Item, path: &Path) -> Result<Option<DocItem>, Box<dyn std::error::Error>> {
    let (doc, name, signature, kind) = match item {
        syn::Item::Struct(s) => {
            let doc = extract_doc_comments(&s.attrs);
            let name = s.ident.to_string();
            let signature = quote!(#s).to_string();
            (doc, name, signature, DocKind::Struct)
        }
        syn::Item::Enum(e) => {
            let doc = extract_doc_comments(&e.attrs);
            let name = e.ident.to_string();
            let signature = quote!(#e).to_string();
            (doc, name, signature, DocKind::Enum)
        }
        syn::Item::Fn(f) => {
            let doc = extract_doc_comments(&f.attrs);
            let name = f.sig.ident.to_string();
            let signature = quote!(#f).to_string();
            (doc, name, signature, DocKind::Function)
        }
        syn::Item::Trait(t) => {
            let doc = extract_doc_comments(&t.attrs);
            let name = t.ident.to_string();
            let signature = quote!(#t).to_string();
            (doc, name, signature, DocKind::Trait)
        }
        syn::Item::Impl(i) => {
            let doc = extract_doc_comments(&i.attrs);
            let name = match &i.trait_ {
                Some((_, path, _)) => quote!(#path).to_string(),
                None => "impl".to_string(),
            };
            let signature = quote!(#i).to_string();
            (doc, name, signature, DocKind::Impl)
        }
        syn::Item::Const(c) => {
            let doc = extract_doc_comments(&c.attrs);
            let name = c.ident.to_string();
            let signature = quote!(#c).to_string();
            (doc, name, signature, DocKind::Constant)
        }
        _ => return Ok(None),
    };

    let location = format!("{}:{}", path.display(), 1); // simplified location
    Ok(Some(DocItem {
        kind,
        name,
        doc,
        signature,
        location,
        children: Vec::new(),
    }))
}

/// Extract doc comments from attributes.
fn extract_doc_comments(attrs: &[syn::Attribute]) -> String {
    let mut docs = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
                    docs.push(s.value());
                }
            }
        }
    }
    docs.join("\n").trim().to_string()
}

/// Generate markdown documentation from parsed items.
pub fn render_markdown(items: &[DocItem], module_name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", module_name));

    for item in items {
        out.push_str(&render_item(item, 1));
    }

    out
}

/// Render a single documentation item at the given heading level.
fn render_item(item: &DocItem, level: usize) -> String {
    let mut out = String::new();
    let heading = "#".repeat(level);
    let kind_label = match item.kind {
        DocKind::Struct => "Struct",
        DocKind::Enum => "Enum",
        DocKind::Function => "Function",
        DocKind::Trait => "Trait",
        DocKind::Impl => "Impl",
        DocKind::Module => "Module",
        DocKind::Constant => "Constant",
    };

    out.push_str(&format!("{} {}\n\n", heading, item.name));
    if !item.doc.is_empty() {
        out.push_str(&format!("{}\n\n", item.doc));
    }
    out.push_str(&format!("**Kind:** {}\n\n", kind_label));
    out.push_str("```rust\n");
    out.push_str(&item.signature);
    out.push_str("\n```\n\n");

    for child in &item.children {
        out.push_str(&render_item(child, level + 1));
    }

    out
}

/// Generate documentation for all Rust files under a directory.
pub fn generate_docs(root: &Path) -> Result<DocOutput, Box<dyn std::error::Error>> {
    let mut all_items = Vec::new();
    let mut crate_name = "unknown".to_string();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let path = entry.path();
        if path.file_name().map_or(true, |n| n != "lib.rs" && n != "main.rs") {
            continue;
        }

        let items = parse_file(path)?;
        if let Some(file_stem) = path.file_stem() {
            if crate_name == "unknown" || file_stem == "lib" {
                crate_name = file_stem.to_string_lossy().to_string();
            }
        }
        all_items.extend(items);
    }

    Ok(DocOutput {
        module_name: crate_name,
        items: all_items,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_doc_comments_works() {
        let attr: syn::Attribute = syn::parse_quote!(#[doc = "Hello world"]);
        let docs = extract_doc_comments(&[attr]);
        assert_eq!(docs, "Hello world");
    }

    #[test]
    fn render_markdown_produces_heading() {
        let items = vec![DocItem {
            kind: DocKind::Struct,
            name: "MyStruct".to_string(),
            doc: "A test struct".to_string(),
            signature: "pub struct MyStruct {}\n".to_string(),
            location: "test.rs:1".to_string(),
            children: Vec::new(),
        }];
        let out = render_markdown(&items, "TestModule");
        assert!(out.contains("# TestModule"));
        assert!(out.contains("# MyStruct"));
        assert!(out.contains("A test struct"));
    }

    #[test]
    fn doc_item_kind_labels() {
        assert_eq!(format!("{:?}", DocKind::Struct), "Struct");
        assert_eq!(format!("{:?}", DocKind::Function), "Function");
    }
}
