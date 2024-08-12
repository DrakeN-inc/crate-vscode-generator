extern crate vscode_generator;
use vscode_generator::{ prelude::* , Package, Snippets, Snippet, License };

#[test]
fn main() -> Result<()> {
    // generating package:
    let pkg = Package::snippets(
        "vscode_rust_snippets",
        "VSCode Rust snippets",
        "The snippets for Rust programming language",
        "0.0.1".parse()?,
        "images/icon.png",
        Some("https://github.com/DrakeN-inc/vscode-rust-snippets"),
        vec![
            // TEXT SNIPPETS:
            Snippets::new(
                "rust",
                "Text",
                "The simple text snippets",
                vec![
                    Snippet::text("print-hello", "hello", r#"println!("Hello, world!");  // TEMP:"#),
                    Snippet::comment("comment-todo", "TODO"),
                    Snippet::comment("comment-note", "NOTE"),
                    Snippet::comment("comment-debug", "DEBUG"),
                    Snippet::comment("comment-fixme", "FIXME"),
                ]
            ),

            // ATTRIBUTES SNIPPETS:
            Snippets::new(
                "rust",
                "Attributes",
                "The attributes snippets",
                vec![
                    Snippet::attribute("attr-derive", "derive", Some(vec![
                        "Debug",
                        "Display",
                        "Clone",
                        "Copy",
                        "Eq\\, PartialEq",
                        "Serialize\\, Deserialize",
                    ])),

                    Snippet::attribute("attr-allow", "allow", Some(vec![
                        "dead_code",
                        "unused_variables",
                        "non_snake_case",
                    ])),

                    Snippet::attribute("attr-cfg", "cfg", Some(vec![""])),
                    Snippet::attribute("attr-test", "test", None),
                ]
            ),

            // BLOCKS SNIPPETS:
            Snippets::new(
                "rust",
                "Blocks",
                "The block snippets",
                vec![
                    Snippet::block("block-struct", "struct"),
                    Snippet::block("block-enum", "enum"),
                    Snippet::double_block("block-struct-impl", "struct", "impl"),
                    Snippet::double_block("block-enum-impl", "enum", "impl"),

                    Snippet::function_block("block-fn", "fn"),
                    
                    Snippet::block("block-if", "if"),
                    Snippet::double_block("block-if-else", "if", "else")
                        .set_descr("if ... { ... } else { ... }")
                        .set_body(vec![
                            "if $1 {",
                            "    $2",
                            "} else {",
                            "    $3",
                            "}",
                        ]),
                    Snippet::block("block-short-if-else", "if?")
                        .set_descr("if ... { ... }else{ ... }")
                        .set_prefix("if?")
                        .set_body(vec![
                            "if $1 { $2 }else{ $3 }",
                        ]),

                    Snippet::block("block-match", "match"),
                    Snippet::block("block-match-result", "match Result<T, E>")
                        .set_descr("match Result<T, E> { ... }")
                        .set_body(vec![
                            "match $1 {",
                            "    Ok(r) => $2,",
                            "    Err(e) => $3,",
                            "}",
                        ]),
                    Snippet::block("block-match-option", "match Option<T>")
                        .set_descr("match Option<T> { ... }")
                        .set_body(vec![
                            "match $1 {",
                            "    Some(r) => $2,",
                            "    None => $3,",
                            "}",
                        ]),

                    Snippet::block("block-while", "while"),

                    Snippet::block("block-for", "for")
                        .set_descr("for ... in ... {}")
                        .set_body(vec![
                            "for $1 in $2 {",
                            "    $3",
                            "}",
                        ]),
                    Snippet::block("block-for-k-v", "for")
                        .set_prefix("for (k, v) in ... {}")
                        .set_descr("for (k, v) in ... {}")
                        .set_body(vec![
                            "for (${1:k, v}) in ${2:.enumerate()} {",
                            "    $3",
                            "}",
                        ]),

                    Snippet::simple_block("block-loop", "loop"),
                ]
            ),

            // OPERATORS SNIPPETS:
            Snippets::new(
                "rust",
                "Operators",
                "The operator snippets",
                vec![
                    Snippet::operator("operator-mod", "mod", Some("")),
                    Snippet::operator("operator-use", "use", Some(""))
                        .set_body(vec!["use $1::$2;"]),
                    Snippet::operator("operator-mod-use", "mod use", Some(""))
                        .set_descr("mod ...;  use ...::...;")
                        .set_body(vec!["mod $1;  pub use $1::$2;"]),

                    Snippet::operator("operator-return", "return", Some("")),
                    Snippet::operator("operator-break", "break", Some("")),
                    Snippet::operator("operator-break", "break", None),
                    Snippet::operator("operator-continue", "continue", None),
                ]
            ),

            // FUNCTIONS & METHODS SNIPPETS:
            Snippets::new(
                "rust",
                "Functions",
                "The functions and methods snippets",
                vec![
                    Snippet::function("fn-read_to_string", "read_to_string", None, None),
                    Snippet::function("fn-write_all", "write_all", None, None),

                    Snippet::function("fn-unwrap", ".unwrap", None, None),
                    Snippet::function("fn-unwrap_or", ".unwrap_or", None, Some("")),
                    Snippet::function("fn-unwrap_or_else", ".unwrap_or_else", None, Some("")),
                    Snippet::function("fn-map_err", ".map_err", None, Some("Error::from")),

                    Snippet::function("fn-new", "::new", None, Some("")),
                    Snippet::function("fn-from", "::from", None, Some("")),

                    Snippet::function("macro-vec", "vec!", Some(("[", "]")), Some("")),
                    Snippet::function("macro-map", "map!", Some(("{", "}")), Some("")),
                ]
            ),
        ],
        License::mit("DrakeN-inc"),
    );

    // writing package files to directory "package/":
    pkg.write_to("package").unwrap();

    Ok(())
}
