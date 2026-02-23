mod fixture;

use assert_cmd::cargo::cargo_bin_cmd;

/// Run the binary with stdin input, --color=never, and assert stdout matches expected.
fn assert_cli(input: String, expected: String) {
    cargo_bin_cmd!("as-tree")
        .args(["--color", "never"])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
}

macro_rules! cli_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let (input, expected) = fixture::load(stringify!($name));
            assert_cli(input, expected);
        }
    };
}

cli_test!(as_tree_simple);
cli_test!(collapse);
cli_test!(double_top_level);
cli_test!(empty);
cli_test!(find_dot);
cli_test!(sorbet);
cli_test!(sorbet_extension_c_h_cc_hh);
cli_test!(sorbet_extension_md);
cli_test!(sorbet_shuf);
cli_test!(symbol);

#[test]
fn full_paths() {
    let (input, expected) = fixture::load("full_paths");
    cargo_bin_cmd!("as-tree")
        .args(["--color", "never", "--path-format", "absolute"])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn reads_from_file() {
    let (_, expected) = fixture::load("as_tree_simple");
    cargo_bin_cmd!("as-tree")
        .args(["--color", "never", "tests/fixture/input/as_tree_simple.txt"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn missing_file_fails() {
    cargo_bin_cmd!("as-tree")
        .args(["--color", "never", "nonexistent.txt"])
        .assert()
        .failure();
}
