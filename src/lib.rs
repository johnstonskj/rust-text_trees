/*!
Simple textual output for tree-like structures.

This crate is another that will output a tree structure in text. Similar to the existing
[ascii_tree](https://crates.io/crates/ascii_tree) crate, however it is more flexible in
its formatting options.

This crate provides a generic [`TreeNode`](struct.TreeNode.html) so that items are stored in the
tree as long as the item implements `std::fmt::Display`. Commonly however a tree is generated
separately from the data it represents and so the simple [`StringTreeNode`](type.StringTreeNode.html)
allows construction of trees with string data throughout. The output trees are generated by writing
to an implementation of `std::io::Write` and helper functions are provided that write to, and
return, a `String`.

The goal is to make not only the writing of the tree easy but the construction should support not
only creating a tree as a stand-alone structure or as a representation of another structure. So,
where options _may_ be provided these option structures implement `Default` and methods that do, and
do not, take options are provided. Additionally, implementations of `From` are provided to simplify
the creation of the tree itself.

# Example

The following example constructs a tree using the `StringTreeNode` type and a combination of
[`with_child_nodes`](struct.TreeNode.html#method.with_child_nodes) and
[`with_children`](struct.TreeNode.html#method.with_children). This demonstrates the structure of
the tree well.

For a more complete example, see the included
[tls](https://github.com/johnstonskj/rust-text_trees/blob/master/examples/tls.rs) _tree ls_ source.

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

# Formatting Options

The format of the output tree is specified by the structure [`TreeFormatting`](struct.TreeFormatting.html)
with a simple format available using the [`dir_tree`](struct.TreeFormatting.html#method.dir_tree)
and [`dir_tree_left`](struct.TreeFormatting.html#method.dir_tree_left) associated functions. You may
also override the default characters used to draw the tree lines with the
[`FormatCharacters`](struct.FormatCharacters.html) structure. Two common sets can be made using the
[`ascii`](struct.FormatCharacters.html#method.ascii) or
[`box_chars`](struct.FormatCharacters.html#method.box_chars) associated functions.

The following sections demonstrate how different combinations of values for the `TreeFormatting` and
`FormtCharacters` structures will affect the output.

| Option          | Setting   |
|-----------------|-----------|
| Character Set   | [`ASCII`](struct.FormatCharacters.html#method.ascii)     |
| Orientation     | [`TopDown`](enum.TreeOrientation.html#variant.TopDown) |
| Anchor Position | [`Below`](enum.AnchorPosition.html#variant.Below)   |
| Prefix String   | `None`    |

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

| Option          | Setting   |
|-----------------|-----------|
| Character Set   | [`ASCII`](struct.FormatCharacters.html#method.ascii)     |
| Orientation     | [`TopDown`](enum.TreeOrientation.html#variant.TopDown) |
| Anchor Position | [`Left`](enum.AnchorPosition.html#variant.Left)    |
| Prefix String   | `None`    |

```text
+ root
+--- Uncle
+--, Parent
|  +--, Child 1
|  |  '--- Grand Child 1
|  '--, Child 2
|     '--, Grand Child 2
|        '--, Great Grand Child 2
|           '--- Great Great Grand Child 2
'--, Aunt
   '--- Child 3
```

| Option          | Setting   |
|-----------------|-----------|
| Character Set   | [`Box char`](struct.FormatCharacters.html#method.box_chars)  |
| Orientation     | [`TopDown`](enum.TreeOrientation.html#variant.TopDown) |
| Anchor Position | [`Below`](enum.AnchorPosition.html#variant.Below)   |
| Prefix String   | `None`    |

```text
root
├── Uncle
├── Parent
│   ├── Child 1
│   │   └── Grand Child 1
│   └── Child 2
│       └── Grand Child 2
│           └── Great Grand Child 2
│               └── Great Great Grand Child 2
└── Aunt
    └── Child 3
```

| Option          | Setting   |
|-----------------|-----------|
| Character Set   | [`Box char`](struct.FormatCharacters.html#method.box_chars)  |
| Orientation     | [`TopDown`](enum.TreeOrientation.html#variant.TopDown) |
| Anchor Position | [`Left`](enum.AnchorPosition.html#variant.Left)    |
| Prefix String   | `None`    |

```text
┌ root
├─── Uncle
├──┬ Parent
│  ├──┬ Child 1
│  │  └─── Grand Child 1
│  └──┬ Child 2
│     └──┬ Grand Child 2
│        └──┬ Great Grand Child 2
│           └─── Great Great Grand Child 2
└──┬ Aunt
   └─── Child 3
```

The following example overrides the basic values of the box character formatting characters to
allow for visualization of the spaces generated. The horizontal spaces are shown as "#" and
the label spacing is shown as ".".

| Option          | Setting   |
|-----------------|-----------|
| Character Set   | Custom    |
| Orientation     | [`TopDown`](enum.TreeOrientation.html#variant.TopDown) |
| Anchor Position | [`Left`](enum.AnchorPosition.html#variant.Left)    |
| Prefix String   | `">> "`   |

```text
>> ┌..root
>> ├──────..Uncle
>> ├─────┬..Parent
>> │#####├─────┬..Child 1
>> │#####│#####└──────..Grand Child 1
>> │#####└─────┬..Child 2
>> │###########└─────┬..Grand Child 2
>> │#################└─────┬..Great Grand Child 2
>> │#######################└──────..Great Great Grand Child 2
>> └─────┬..Aunt
>> ######└──────..Child 3

*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

use std::fmt::{Display, Formatter};
use std::io::Result;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This denotes the orientation of the tree as it is written.
///
#[derive(Clone, Debug, PartialEq)]
pub enum TreeOrientation {
    /// This writes a tree with the root node at the top-left corner and the tree expanding
    /// out to the right and down. This is often called a _directory tree_ as it is useful in
    /// showing file-system hierarchy.
    ///
    /// # Example
    ///
    /// ```text
    /// root
    /// +-- Uncle
    /// +-- Parent
    /// |  +-- Child 1
    /// |  |  '-- Grand Child 1
    /// |  '-- Child 2
    /// |     '-- Grand Child 2
    /// |        '-- Great Grand Child 2
    /// |           '-- Great Great Grand Child 2
    /// '-- Aunt
    /// '-- Child 3
    /// ```
    ///
    TopDown,
}

///
/// Denotes the position where the generated tree lines are anchored to the label text.
#[derive(Clone, Debug, PartialEq)]
pub enum AnchorPosition {
    /// The line is anchored below the first letter of the label.
    ///
    /// # Example
    ///
    /// ```text
    /// parent_node
    /// '-- child_node
    /// ```
    Below,
    /// The line is anchored to the left of the label, and the label spacing prefix.
    ///
    /// # Example
    ///
    /// ```text
    /// + parent_node
    /// '--- child_node
    /// ```
    Left,
}

///
/// This structure collects together all the formatting options that control how the tree is
/// output.
///
#[derive(Clone, Debug)]
pub struct TreeFormatting {
    /// A prefix string written before every line. While no validation is performed on this value,
    /// if newline (or other formatting) characters are included the tree is likely to appear
    /// disjointed.
    pub prefix_str: Option<String>,
    /// The orientation to write the tree.
    pub orientation: TreeOrientation,
    /// The line anchor position.
    pub anchor: AnchorPosition,
    /// The set of characters to use when line formatting.
    pub chars: FormatCharacters,
}

///
/// Contains the set of characters, and counts, to use when line formatting.
///
#[derive(Clone, Debug)]
pub struct FormatCharacters {
    /// This character is used to connect the root of the tree when line anchors are on the left.
    /// ASCII value `'+'`, box character value `'┌'`.
    pub down_facing_angle: char,

    /// This character is used to connect non-root parents in the tree when line anchors are on the left.
    /// ASCII value `','`, box character value `'┬'`.
    pub down_facing_tee: char,

    /// This character is used as the vertical connector between parent and child nodes.
    /// ASCII value `'|'`, box character value `'│'`.
    pub vertical_line: char,

    /// This character is used as the horizontal connector to node labels.
    /// ASCII value `'-'`, box character value `'─'`.
    pub horizontal_line: char,

    /// The character to use instead of `horizontal_line` where lines are not present.
    /// ASCII value `' '`, box character value `' '`.
    pub horizontal_space: char,

    /// The number of `horizontal_line`, or `horizontal_space` characters connecting lines to labels.
    pub horizontal_line_count: usize,

    /// This character is used to connect non-terminal child nodes.
    /// ASCII value `'+'`, box character value `'├'`.
    pub right_facing_tee: char,

    /// This character is used to connect terminal child nodes.
    /// ASCII value `'\''`, box character value `'└'`.
    pub right_facing_angle: char,

    /// This character is used as the spacing between the lines of the tree and the labels of each node.
    /// ASCII value `' '`, box character value `' '`.
    pub label_space_char: char,

    /// The number of `label_space_char` characters between the lines of the tree and the labels of each node.
    /// ASCII value `''`, box character value `''`.
    pub label_space_count: usize,
}

///
/// Denotes a node in the tree, and any node can be the root of a tree when output. The generic
/// parameter `T` must implement `Display` which is used to generate the label for each node in
/// the output.
///
/// Note that `From<T>` is implemented allowing a nice short-cut for node creation, and `From<&T>`
/// is also implemented for types that also implement `Clone`.
///
#[derive(Clone, Debug)]
pub struct TreeNode<T>
where
    T: Display,
{
    data: T,
    children: Vec<TreeNode<T>>,
}

///
/// A common type where the only data is the node's label as a `String`.
///
/// Note that `From<&str> is implemented for `TreeNode<String>`.
///
pub type StringTreeNode = TreeNode<String>;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for TreeFormatting {
    fn default() -> Self {
        Self::dir_tree(Default::default())
    }
}

impl TreeFormatting {
    /// Construct the common options for a directory tree using the provided format characters.
    pub fn dir_tree(chars: FormatCharacters) -> Self {
        Self {
            prefix_str: None,
            orientation: TreeOrientation::TopDown,
            anchor: AnchorPosition::Below,
            chars,
        }
    }

    /// Construct the common options for a directory tree using the provided format characters.
    /// Additionally, the value for `prefix_str` will be used for each output line.
    pub fn dir_tree_with_prefix(chars: FormatCharacters, prefix_str: String) -> Self {
        Self {
            prefix_str: Some(prefix_str),
            orientation: TreeOrientation::TopDown,
            anchor: AnchorPosition::Below,
            chars,
        }
    }

    /// Construct the common options for a directory tree, with lines anchored to the left, using
    /// the provided format characters.
    pub fn dir_tree_left(chars: FormatCharacters) -> Self {
        Self {
            prefix_str: None,
            orientation: TreeOrientation::TopDown,
            anchor: AnchorPosition::Left,
            chars,
        }
    }

    /// Construct the common options for a directory tree, with lines anchored to the left, using
    /// the provided format characters. Additionally, the value for `prefix_str` will be used for
    /// each output line.
    pub fn dir_tree_left_with_prefix(chars: FormatCharacters, prefix_str: String) -> Self {
        Self {
            prefix_str: Some(prefix_str),
            orientation: TreeOrientation::TopDown,
            anchor: AnchorPosition::Left,
            chars,
        }
    }

    #[inline]
    pub(crate) fn just_space(&self) -> String {
        format!(
            "{}{}",
            self.chars.just_space(),
            if self.anchor == AnchorPosition::Below && self.chars.label_space_count > 0 {
                self.chars.horizontal_space.to_string()
            } else {
                String::new()
            }
        )
    }

    #[inline]
    pub(crate) fn bar_and_space(&self) -> String {
        format!(
            "{}{}",
            self.chars.bar_and_space(),
            if self.anchor == AnchorPosition::Below && self.chars.label_space_count > 0 {
                self.chars.horizontal_space.to_string()
            } else {
                String::new()
            }
        )
    }

    #[inline]
    pub(crate) fn tee(&self, has_children: bool) -> String {
        format!(
            "{}{}{}{}",
            self.chars.right_facing_tee,
            self.chars.horizontal_line(),
            if self.anchor == AnchorPosition::Below {
                String::new()
            } else if has_children {
                self.chars.down_facing_tee.to_string()
            } else {
                self.chars.horizontal_line.to_string()
            },
            self.chars.label_space()
        )
    }

    #[inline]
    pub(crate) fn angle(&self, has_children: bool) -> String {
        format!(
            "{}{}{}{}",
            self.chars.right_facing_angle,
            self.chars.horizontal_line(),
            if self.anchor == AnchorPosition::Below {
                String::new()
            } else if has_children {
                self.chars.down_facing_tee.to_string()
            } else {
                self.chars.horizontal_line.to_string()
            },
            self.chars.label_space(),
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for FormatCharacters {
    fn default() -> Self {
        Self::ascii()
    }
}

impl FormatCharacters {
    /// The set of commonly used ASCII characters used for tree formatting.
    pub fn ascii() -> Self {
        Self {
            down_facing_angle: '+',
            down_facing_tee: ',',
            vertical_line: '|',
            horizontal_line: '-',
            horizontal_space: ' ',
            horizontal_line_count: 2,
            right_facing_tee: '+',
            right_facing_angle: '\'',
            label_space_char: ' ',
            label_space_count: 1,
        }
    }

    /// The set of commonly used line drawing characters used for tree formatting.
    pub fn box_chars() -> Self {
        Self {
            down_facing_angle: '┌',
            down_facing_tee: '┬',
            vertical_line: '│',
            horizontal_line: '─',
            horizontal_space: ' ',
            horizontal_line_count: 2,
            right_facing_tee: '├',
            right_facing_angle: '└',
            label_space_char: ' ',
            label_space_count: 1,
        }
    }

    #[inline]
    pub(crate) fn just_space(&self) -> String {
        format!("{}{}", self.horizontal_space, self.horizontal_space(),)
    }

    #[inline]
    pub(crate) fn bar_and_space(&self) -> String {
        format!("{}{}", self.vertical_line, self.horizontal_space(),)
    }

    #[inline]
    pub(crate) fn horizontal_line(&self) -> String {
        char_repeat(self.horizontal_line, self.horizontal_line_count)
    }

    #[inline]
    pub(crate) fn horizontal_space(&self) -> String {
        char_repeat(self.horizontal_space, self.horizontal_line_count)
    }

    #[inline]
    pub(crate) fn label_space(&self) -> String {
        char_repeat(self.label_space_char, self.label_space_count)
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> Display for TreeNode<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_with_format(&Default::default()).unwrap()
        )
    }
}

impl<T> TreeNode<T>
where
    T: Display,
{
    /// Construct a new tree node with the provided data value.
    pub fn new(data: T) -> Self {
        Self {
            data,
            children: Default::default(),
        }
    }

    /// Construct a new tree node with the provided data value and an iterator that provides
    /// child data items.
    pub fn with_children(data: T, children: impl Iterator<Item = T>) -> Self
    where
        T: Sized,
    {
        Self::with_child_nodes(data, children.map(TreeNode::new))
    }

    /// Construct a new tree node with the provided data value and an iterator that provides
    /// pre-constructed `TreeNode` values as child nodes.
    pub fn with_child_nodes(data: T, children: impl Iterator<Item = TreeNode<T>>) -> Self
    where
        T: Sized,
    {
        Self {
            data,
            children: children.collect(),
        }
    }

    /// Return a reference to the data item for this node.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Return the label for this node.
    pub fn label(&self) -> String {
        self.data.to_string()
    }

    /// Returns `true` if this node has child nodes, else `false`.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Returns an iterator that will return all the child nodes.
    pub fn children(&self) -> impl Iterator<Item = &TreeNode<T>> {
        self.children.iter()
    }

    /// Push a new data item into the list of children.
    pub fn push(&mut self, data: T) {
        self.push_node(TreeNode {
            data,
            children: Default::default(),
        })
    }

    /// Push a new pre-constructed `TreeNode` into the list of children.
    pub fn push_node(&mut self, child: TreeNode<T>) {
        self.children.push(child)
    }

    /// Extend the list of children with each data item from the provided iterator.
    pub fn extend<V>(&mut self, children: impl Iterator<Item = T>) {
        self.children.extend(children.map(TreeNode::new))
    }

    ///
    /// Return a string containing the generated tree text formatted according to the provided
    /// format settings.
    ///
    /// _Note_: in effect `Display::fmt` calls this method with default formatting.
    ///  
    pub fn to_string_with_format(&self, format: &TreeFormatting) -> Result<String> {
        use std::io::Cursor;
        let mut buffer = Cursor::new(Vec::new());
        self.write_with_format(&mut buffer, format)?;
        Ok(String::from_utf8(buffer.into_inner()).unwrap())
    }

    /// Write this tree to the provided implementation of `std::io::Write` with default formatting.
    pub fn write(&self, to_writer: &mut impl Write) -> Result<()>
    where
        T: Display,
    {
        self.write_with_format(
            to_writer,
            &TreeFormatting::dir_tree(FormatCharacters::ascii()),
        )
    }

    /// Write this tree to the provided implementation of `std::io::Write` with the provided
    /// format settings.
    pub fn write_with_format(
        &self,
        to_writer: &mut impl Write,
        format: &TreeFormatting,
    ) -> Result<()>
    where
        T: Display,
    {
        write_tree_inner(self, to_writer, format, Default::default())
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> From<T> for TreeNode<T>
where
    T: Display,
{
    fn from(v: T) -> Self {
        Self {
            data: v,
            children: Default::default(),
        }
    }
}

impl<T> From<&T> for TreeNode<T>
where
    T: Display + Clone,
{
    fn from(v: &T) -> Self {
        Self::from(v.clone())
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> PartialEq for TreeNode<T>
where
    T: Display + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.children == other.children
    }
}

// ------------------------------------------------------------------------------------------------

impl From<&str> for TreeNode<String> {
    fn from(v: &str) -> Self {
        Self {
            data: v.to_string(),
            children: Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_tree_inner<T>(
    node: &TreeNode<T>,
    w: &mut impl Write,
    format: &TreeFormatting,
    remaining_children_stack: Vec<usize>,
) -> Result<()>
where
    T: Display,
{
    // Write any requested prefix
    if let Some(prefix_str) = &format.prefix_str {
        write!(w, "{}", prefix_str)?;
    }

    if !(format.anchor == AnchorPosition::Below) && remaining_children_stack.is_empty() {
        write!(
            w,
            "{}{}",
            format.chars.down_facing_angle,
            char_repeat(
                format.chars.label_space_char,
                format.chars.label_space_count
            )
        )?;
    }

    // Write the leading structures
    let stack_depth = remaining_children_stack.len();
    for (row, remaining_children) in remaining_children_stack.iter().enumerate() {
        write!(
            w,
            "{}",
            match (*remaining_children, row == (stack_depth - 1)) {
                (1, true) => format.angle(node.has_children()),
                (1, false) => format.just_space(),
                (_, true) => format.tee(node.has_children()),
                (_, false) => format.bar_and_space(),
            }
        )?;
    }

    // Write the node label, and any children (recursively)
    if node.has_children() {
        writeln!(w, "{}", node.label())?;
        let mut d = node.children.len();
        for child in &node.children {
            let mut new_child_stack = remaining_children_stack.clone();
            new_child_stack.push(d);
            d -= 1;
            write_tree_inner(child, w, format, new_child_stack)?;
        }
    } else {
        writeln!(w, "{}", node.label())?;
    }

    // All done :)
    Ok(())
}

#[inline]
fn char_repeat(c: char, n: usize) -> String {
    c.to_string().as_str().repeat(n)
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let node = TreeNode::new(String::from("hello"));
        assert_eq!(
            node,
            TreeNode {
                data: "hello".to_string(),
                children: vec![]
            }
        );
    }

    #[test]
    fn test_node_with_children() {
        let node = TreeNode::with_children(String::from("hello"), vec!["world".into()].into_iter());
        assert_eq!(
            node,
            TreeNode {
                data: "hello".to_string(),
                children: vec![TreeNode {
                    data: "world".to_string(),
                    children: vec![]
                }]
            }
        );
    }

    #[test]
    fn test_node_from_string() {
        let node: TreeNode<String> = String::from("hello").into();
        assert_eq!(
            node,
            TreeNode {
                data: "hello".to_string(),
                children: vec![]
            }
        );
    }
}
