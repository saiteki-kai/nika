use snapbox::cmd::{cargo_bin, Command};

#[test]
fn test_cli_info() {
    Command::new(cargo_bin!("nika"))
        .arg("--help")
        .assert()
        .success();

    Command::new(cargo_bin!("nika"))
        .arg("--version")
        .assert()
        .success();
}
