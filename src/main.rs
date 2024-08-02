extern crate vscode_generator;
use vscode_generator::{ prelude::* , Package, Snippets, Snippet, License };

fn main() -> Result<()> {
    // generating package:
    let pkg = Package::snippets(
        "vscode_rust_snippets",
        "VSCode Rust snippets",
        "The Rust snippets",
        "The snippets for Rust programming language",
        "0.0.1".parse()?,
        "images/icon.png",
        Some("https://github.com/DrakeN-inc/vscode-rust-snippets"),
        vec![
            Snippets::new(
                "rust",
                "text",
                "The simple text snippets",
                vec![
                    Snippet::text("print-hello", "hello", r#"println!("Hello, world!");  // TEMP:"#),
                ]
            )
        ],
        License::mit("DrakeN-inc"),
    );

    // writing package files to directory "package/":
    pkg.write_to("package")?;

    Ok(())
}
