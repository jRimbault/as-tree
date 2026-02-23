mod trie;

pub use trie::PathTrie;

/// Specify how to display paths
#[derive(Copy, Clone, Debug, Default)]
pub enum PathFormat {
    #[default]
    Normal,
    Absolute,
}

impl From<bool> for PathFormat {
    fn from(value: bool) -> Self {
        if value {
            PathFormat::Absolute
        } else {
            PathFormat::Normal
        }
    }
}

impl std::str::FromStr for PathFormat {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "absolute" => Ok(PathFormat::Absolute),
            _ => Ok(PathFormat::Normal),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn path_format_from_bool() {
        assert!(matches!(PathFormat::from(true), PathFormat::Absolute));
        assert!(matches!(PathFormat::from(false), PathFormat::Normal));
    }

    #[test]
    fn path_format_from_str() {
        assert!(matches!(
            "absolute".parse::<PathFormat>().unwrap(),
            PathFormat::Absolute
        ));
        assert!(matches!(
            "normal".parse::<PathFormat>().unwrap(),
            PathFormat::Normal
        ));
        // Unknown values fall back to Normal
        assert!(matches!(
            "bogus".parse::<PathFormat>().unwrap(),
            PathFormat::Normal
        ));
    }

    #[test]
    fn path_format_default_is_normal() {
        assert!(matches!(PathFormat::default(), PathFormat::Normal));
    }
}
