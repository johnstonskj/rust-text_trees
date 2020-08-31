# Crate text_trees

Simple textual output for tree-like structures.

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

The following creates a `StringTreeNode` using a combination of `with_child_nodes` and
`with_children` that demonstrates the structure of the tree well.

```rust
use text_trees::StringTreeNode;

fn make_tree() -> StringTreeNode {
    StringTreeNode::with_child_nodes(
        "root".to_string(),
        vec![
            "Uncle".into(),
            StringTreeNode::with_child_nodes(
                "Parent".to_string(),
                vec![
                    StringTreeNode::with_children(
                        "Child 1".to_string(),
                        vec!["Grand Child 1".into()].into_iter(),
                    ),
                    StringTreeNode::with_child_nodes(
                        "Child 2".to_string(),
                        vec![StringTreeNode::with_child_nodes(
                            "Grand Child 2".to_string(),
                            vec![StringTreeNode::with_children(
                                "Great Grand Child 2".to_string(),
                                vec!["Great Great Grand Child 2".to_string()].into_iter(),
                            )]
                            .into_iter(),
                        )]
                        .into_iter(),
                    ),
                ]
                .into_iter(),
            ),
            StringTreeNode::with_children(
                "Aunt".to_string(),
                vec!["Child 3".to_string()].into_iter(),
            ),
        ]
        .into_iter(),
    )
}
```

The tree implements `Display` and therefore provides a `to_string` method. It also has a 
`to_string_with_format` method that allows for customization of the output format. Finally, it
has two _write_ methods that take implementations of `std::io::Write` and will serialize accordingly.

```rust
use text_trees::{FormatCharacters, TreeFormatting, TreeNode};

fn ascii_tree(tree: TreeNode<String>) {
    let result = tree.to_string_with_format(
        &TreeFormatting::dir_tree(FormatCharacters::ascii())
    );
    assert!(result.is_ok());

    // ... do something else
}
```

This results in a textual representation of the tree as follows.

```text
root
+-- Uncle
+-- Parent
|   +-- Child 1
|   |   '-- Grand Child 1
|   '-- Child 2
|       '-- Grand Child 2
|           '-- Great Grand Child 2
|               '-- Great Great Grand Child 2
'-- Aunt
   '-- Child 3
```

# Changes

**Version 0.1.1**

* Bug in top-down, bottom-anchored, tree with missing spacing.
* Updated all examples to match the tree output changes.
* Added `tls` tree-ls example.

**Version 0.1.0**

* Initial version, supports only _directory_ style trees.

# TODO

TBD