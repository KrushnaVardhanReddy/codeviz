use std::fs;

#[test]
fn test_resolve_import_path_with_mod() {
    // We simulate creating a go.mod
    let dir = tempfile::tempdir().unwrap();
    let mod_path = dir.path().join("go.mod");
    fs::write(&mod_path, "module github.com/user/myapp\n\ngo 1.21").unwrap();

    let root_str = dir.path().to_str().unwrap();

    let resolved = crate::parser::resolve_import_path("github.com/user/myapp/pkg/parser", root_str);
    assert_eq!(resolved, "pkg/parser");

    let resolved_external = crate::parser::resolve_import_path("github.com/other/pkg", root_str);
    assert_eq!(resolved_external, "github.com/other/pkg");
}
