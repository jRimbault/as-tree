use super::PathFormat;
use lscolors::{LsColors, Style};
use std::ffi::OsString;
use std::fmt;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

/// Collection of [paths](std::path).
///
/// # Examples :
///
/// ```
/// use as_tree::PathTrie;
///
/// let trie: PathTrie = "".lines().collect();
/// let trie: PathTrie = [""].iter().collect();
/// ```
#[derive(Debug, Default)]
pub struct PathTrie {
    nodes: Vec<RadixNode>,
}

#[derive(Debug)]
struct RadixNode {
    /// Single path segment
    segment: OsString,
    /// Child nodes, kept sorted by segment for binary search
    children: Vec<RadixNode>,
}

impl PathTrie {
    /// Adds a path to the internal collection.
    pub fn insert<P: AsRef<Path>>(&mut self, path: P) {
        let components: Vec<OsString> = path.as_ref().iter().map(|c| c.to_os_string()).collect();

        if components.is_empty() {
            return;
        }

        PathTrie::insert_components_impl(&components, 0, &mut self.nodes);
    }

    fn insert_components_impl(
        components: &[OsString],
        start_idx: usize,
        nodes: &mut Vec<RadixNode>,
    ) {
        if start_idx >= components.len() {
            return;
        }

        let component = &components[start_idx];

        // Binary search for matching node
        let pos = nodes
            .binary_search_by(|node| node.segment.cmp(component))
            .unwrap_or_else(|e| e);

        if let Ok(idx) = nodes.binary_search_by(|node| node.segment.cmp(component)) {
            // Found existing node - continue insertion in children
            PathTrie::insert_components_impl(components, start_idx + 1, &mut nodes[idx].children);
        } else {
            // Create new node
            let new_node = RadixNode {
                segment: component.clone(),
                children: Vec::new(),
            };
            nodes.insert(pos, new_node);

            // Continue inserting remaining components
            if start_idx + 1 < components.len() {
                PathTrie::insert_components_impl(
                    components,
                    start_idx + 1,
                    &mut nodes[pos].children,
                );
            }
        }
    }

    fn contains_singleton_dir(&self) -> bool {
        self.nodes.len() == 1 && !self.nodes[0].children.is_empty()
    }

    fn node_contains_singleton_dir(node: &RadixNode) -> bool {
        node.children.len() == 1 && !node.children[0].children.is_empty()
    }

    /// Returns a struct that implements [`Display`](std::fmt::Display) for printing.
    pub fn display(&self) -> DisplayTrie<'_> {
        DisplayTrie::new(self, LsColors::empty(), PathFormat::Normal)
    }

    /// Returns a struct that implements [`Display`](std::fmt::Display) for printing,
    /// with colors and custom formatting.
    pub fn custom_display(&self, colors: LsColors, path_format: PathFormat) -> DisplayTrie<'_> {
        DisplayTrie::new(self, colors, path_format)
    }
}

impl<P: AsRef<Path>> FromIterator<P> for PathTrie {
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        let mut trie = Self::default();
        for path in iter {
            trie.insert(path);
        }
        trie
    }
}

/// Helper struct for safely displaying a [PathTrie](structs.PathTrie)
#[derive(Debug)]
pub struct DisplayTrie<'a> {
    path_trie: &'a PathTrie,
    options: DisplayOptions,
}

#[derive(Debug)]
struct DisplayOptions {
    path_format: PathFormat,
    colors: LsColors,
    colors_enabled: bool,
}

impl<'a> DisplayTrie<'a> {
    pub fn new(path_trie: &'a PathTrie, colors: LsColors, path_format: PathFormat) -> Self {
        // Check if colors are actually configured by testing a simple path
        let colors_enabled = colors.style_for_path(".").is_some();
        Self {
            path_trie,
            options: DisplayOptions {
                path_format,
                colors,
                colors_enabled,
            },
        }
    }
}

impl<'a> fmt::Display for DisplayTrie<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.path_trie.nodes.is_empty() {
            writeln!(f)
        } else {
            let current_path: &Path = ".".as_ref();
            let contains_singleton_dir = self.path_trie.contains_singleton_dir();

            if !contains_singleton_dir {
                let style = ansi_style_for_path(&self.options.colors, current_path);
                writeln!(f, "{}", style.paint(current_path.to_string_lossy()))?;
            }

            fmt::Display::fmt(
                &SubTrie {
                    nodes: &self.path_trie.nodes,
                    parent_path: current_path,
                    prefix: "",
                    is_top: true,
                    join_with_parent: contains_singleton_dir,
                    options: &self.options,
                },
                f,
            )
        }
    }
}

#[derive(Debug)]
struct SubTrie<'a> {
    nodes: &'a [RadixNode],
    parent_path: &'a Path,
    prefix: &'a str,
    is_top: bool,
    join_with_parent: bool,
    options: &'a DisplayOptions,
}

impl<'a> fmt::Display for SubTrie<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let normal_prefix = format!("{}│   ", self.prefix);
        let last_prefix = format!("{}    ", self.prefix);

        for (idx, node) in self.nodes.iter().enumerate() {
            let is_last = idx == self.nodes.len() - 1;
            let current_path = self.parent_path.join(&node.segment);

            let style = if self.options.colors_enabled {
                ansi_style_for_path(&self.options.colors, &current_path)
            } else {
                nu_ansi_term::Style::default()
            };
            let segment_path = PathBuf::from(&node.segment);
            let painted = match self.options.path_format {
                PathFormat::Normal => segment_path.to_string_lossy(),
                PathFormat::Absolute => current_path.to_string_lossy(),
            };
            let painted = style.paint(painted);

            let contains_singleton_dir = PathTrie::node_contains_singleton_dir(node);
            let newline = if contains_singleton_dir { "" } else { "\n" };

            let next_prefix = if self.join_with_parent {
                let joiner = if self.is_top || self.parent_path.as_os_str().len() == 1 {
                    ""
                } else {
                    std::path::MAIN_SEPARATOR_STR
                };
                write!(f, "{}{}{}", style.paint(joiner), painted, newline)?;
                self.prefix
            } else if !is_last {
                write!(f, "{}├── {}{}", self.prefix, painted, newline)?;
                &normal_prefix
            } else {
                write!(f, "{}└── {}{}", self.prefix, painted, newline)?;
                &last_prefix
            };

            fmt::Display::fmt(
                &SubTrie {
                    nodes: &node.children,
                    parent_path: &current_path,
                    prefix: next_prefix,
                    is_top: false,
                    join_with_parent: contains_singleton_dir,
                    options: self.options,
                },
                f,
            )?;
        }

        Ok(())
    }
}

fn ansi_style_for_path<P: AsRef<Path>>(colors: &LsColors, path: P) -> nu_ansi_term::Style {
    colors
        .style_for_path(path) // syscall here
        .map(Style::to_nu_ansi_term_style)
        .unwrap_or_default()
}
