mod fixture;

use as_tree::{PathFormat, PathTrie};
use lscolors::LsColors;

fn assert_display(input: &str, expected: &str) {
    let trie: PathTrie = input.lines().collect();
    let result = format!("{}", trie.display());
    assert_eq!(result, expected);
}

macro_rules! display_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let (input, expected) = fixture::load(stringify!($name));
            assert_display(&input, &expected);
        }
    };
}

display_test!(as_tree_simple);
display_test!(as_tree_absolute);
display_test!(collapse);
display_test!(double_top_level);
display_test!(empty);
display_test!(find_dot);
display_test!(sorbet);
display_test!(sorbet_extension_c_h_cc_hh);
display_test!(sorbet_extension_md);
display_test!(sorbet_shuf);
display_test!(symbol);

/// This test relies on tests/dir1/ and tests/dir2/ existing on disk.
/// LsColors::default() stats real paths to apply directory coloring.
#[test]
fn filesystem() {
    let (input, expected) = fixture::load("filesystem");
    let trie: PathTrie = input.lines().collect();
    let result = format!(
        "{}",
        trie.custom_display(LsColors::default(), PathFormat::Normal)
    );
    assert_eq!(result, expected);
}

#[test]
fn full_paths() {
    let (input, expected) = fixture::load("full_paths");
    let trie: PathTrie = input.lines().collect();
    let result = format!(
        "{}",
        trie.custom_display(LsColors::empty(), PathFormat::Absolute)
    );
    assert_eq!(result, expected);
}
