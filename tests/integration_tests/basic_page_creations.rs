use std::path::Path;

use assert_cmd::Command;

/**
 *  "Basic page creations" are all page creations that does not use any of the
 *  command options.
 *
 *  Before testing, is advised to remove all files and folders from <path/to/crate>/src/app/
 *
 *  As of version 0.1.0, all pages are created based on the app router
 *  by default.
 * */

#[test]
fn test_basic_page_creation() {
    let mut cmd = Command::cargo_bin("next-butler").unwrap();

    let page_to_create = "/my/test";
    cmd.args(["new", "page", page_to_create]);
    cmd.assert().success();
    let expected_page_path = Path::new("src/app/my/test/page.jsx");
    assert!(
        expected_page_path.is_file(),
        "[{}] not created at the correct location",
        page_to_create
    );
}

#[test]
fn test_basic_page_creation_with_parents_dir() {
    let mut cmd = Command::cargo_bin("next-butler").unwrap();

    let page_to_create = "../players/../profile";
    cmd.args(["new", "page", page_to_create]);
    cmd.assert().success();
    let expected_page_path = Path::new("src/app/players/profile/page.jsx");
    assert!(
        expected_page_path.is_file(),
        "[{}] not created at the correct location",
        page_to_create
    );
}

#[test]
fn test_basic_page_creation_without_file() {
    let mut cmd = Command::cargo_bin("next-butler").unwrap();

    let page_to_create = "/users/login/";
    cmd.args(["new", "page", page_to_create]);
    cmd.assert().success();
    let expected_page_path = Path::new("src/app/users/login/page.jsx");
    assert!(
        expected_page_path.is_file(),
        "[{}] not created at the correct location",
        page_to_create
    );
}
