# Crate text_trees

Simple textual output for tree structures.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
![Rust](https://github.com/johnstonskj/rust-text_trees/workflows/Rust/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-text_trees.svg)](https://github.com/johnstonskj/rust-text_trees/stargazers)
[![crates.io](https://img.shields.io/crates/v/text_trees.svg)](https://crates.io/crates/text_trees)
[![docs.rs](https://docs.rs/text_trees/badge.svg)](https://docs.rs/text_trees)

This crate is another that will output a tree structure in text. Similar to the existing
[ascii_tree](https://crates.io/crates/ascii_tree) crate, however it is more flexible in 
its formatting options.

# Example

Tree construction ...

```rust
use text_trees::SimpleTreeNode;

fn make_tree() -> SimpleTreeNode {
    SimpleTreeNode::with_children(
        "root",
        vec![
            "Uncle".into(),
            SimpleTreeNode::with_children(
                "Parent",
                vec![
                    SimpleTreeNode::with_children(
                        "Child 1",
                        vec!["Grand Child 1".into()].into_iter(),
                    ),
                    SimpleTreeNode::with_children(
                        "Child 2".to_string(),
                        vec![SimpleTreeNode::with_children(
                            "Grand Child 2",
                            vec![SimpleTreeNode::with_children(
                                "Great Grand Child 2",
                                vec!["Great Great Grand Child 2".into()].into_iter(),
                            )]
                            .into_iter(),
                        )]
                        .into_iter(),
                    ),
                ]
                .into_iter(),
            ),
            SimpleTreeNode::with_children("Aunt", vec!["Child 3".to_string().into()].into_iter()),
        ]
        .into_iter(),
    )
}
```

Tree output ...

```rust
use text_trees::{ascii_formatting, to_string_with};

fn test_ascii_below_tree() {
    let tree = make_tree();

    let result = to_string_with(&tree, &ascii_formatting(true), None);
    assert!(result.is_ok());
}
```

Results ...

```textroot
       +-- Uncle
       +-- Parent
       |  +-- Child 1
       |  |  '-- Grand Child 1
       |  '-- Child 2
       |     '-- Grand Child 2
       |        '-- Great Grand Child 2
       |           '-- Great Great Grand Child 2
       '-- Aunt
          '-- Child 3
```