use nika::messages::EMPTY_STUDY_LISTS;
use nika::messages::NO_LIST_SELECTED;
use snapbox::cmd::cargo_bin;
use snapbox::cmd::Command;

#[test]
fn test_empty_study_list() {
    let cmd = Command::new(cargo_bin("nika")).arg("study").arg("list");

    cmd.assert()
        .success()
        .stdout_matches(format!("{}\n", EMPTY_STUDY_LISTS));
}

#[test]
fn test_empty_selection() {
    let cmd = Command::new(cargo_bin("nika")).arg("study").arg("daily");

    cmd.assert()
        .failure()
        .code(1)
        .stderr_matches(format!("Error: {}\n", NO_LIST_SELECTED));
}
