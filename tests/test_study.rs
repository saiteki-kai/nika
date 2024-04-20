use nika::messages::DAILY_LIST_EMPTY;
use snapbox::cmd::cargo_bin;
use snapbox::cmd::Command;

#[test]
fn test_empty_study_list() {
    let cmd = Command::new(cargo_bin("nika")).arg("daily").arg("list");

    cmd.assert()
        .success()
        .stdout_matches(format!("{}\n", DAILY_LIST_EMPTY));
}
