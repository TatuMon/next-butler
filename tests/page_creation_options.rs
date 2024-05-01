use std::path::Path;

use assert_cmd::Command;

/**
 *  Before testing, is advised to remove all files and folders from <path/to/crate>/src/app/
 *  and <path/to/crate>/src/pages/
 *
 *  As of version 0.1.0, all pages are created based on the app router
 *  by default.
 * */

#[test]
fn test_page_router_option() {
    let mut cmd = Command::cargo_bin("next-butler").unwrap();

    let page_to_create = "/my/test_router";
    cmd.args(["new", "page", page_to_create, "--page-router"]);
    cmd.assert().success();
    let expected_page_path = Path::new("src/pages/my/test.jsx");
    assert!(
        expected_page_path.is_file(),
        "[{}] not created at the correct location",
        page_to_create
    );
}

/**
 *  Refer to the README to know how to create and use templates
 * */
#[test]
fn test_page_custom_template() {
    let mut cmd = Command::cargo_bin("next-butler").unwrap();

    let page_to_create = "/my/test_template";
    let template_to_use = "test_template";
    cmd.args(["new", "page", page_to_create, "--template", template_to_use]);
    cmd.assert().success();
    let expected_page_path = Path::new("src/app/my/test/page.tsx");
    assert!(
        expected_page_path.is_file(),
        "[{}] not created at the correct location",
        page_to_create
    );
}

#[test]
fn test_page_extension_conflict() {
    let mut cmd = Command::cargo_bin("next-butler").unwrap();

    let page_to_create = "/my/test_ext_conflict";
    cmd.args(["new", "page", page_to_create, "--tsx", "--jsx"]);
    cmd.assert().failure();
}
