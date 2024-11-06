use rust_refactor::refactor;

#[test]
fn test_extract_function() {
    let code = "let x = 5;";
    let extracted = refactor::extract_function(code, "my_func");
    assert_eq!(extracted, "fn my_func() {\nlet x = 5;\n}");
}

#[test]
fn test_rename_variable() {
    let code = "let x = 5; x + 1";
    let renamed = refactor::rename_variable(code, "x", "y");
    assert_eq!(renamed, "let y = 5; y + 1");
}
