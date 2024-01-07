use assert_cmd::Command;

#[test]
fn test_page_creation() {
    let mut cmd = Command::cargo_bin("next-butler").unwrap();
    cmd.args(["new", "page", "/api/test"]);
    cmd.assert().success();
}
