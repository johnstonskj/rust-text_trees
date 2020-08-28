/*!
One-line description.

More detailed description, with

# Example


# Formatting

```text
root
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

```text
root
├── Uncle
├── Parent
│  ├── Child 1
│  │  └── Grand Child 1
│  └── Child 2
│     └── Grand Child 2
│        └── Great Grand Child 2
│           └── Great Great Grand Child 2
└── Aunt
   └── Child 3
```

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

```text
.. ┌..root
.. ├──────..Uncle
.. ├─────┬..Parent
.. │#####├─────┬..Child 1
.. │#####│#####└──────..Grand Child 1
.. │#####└─────┬..Child 2
.. │###########└─────┬..Grand Child 2
.. │#################└─────┬..Great Grand Child 2
.. │#######################└──────..Great Great Grand Child 2
.. └─────┬..Aunt
.. ######└──────..Child 3

*/

use std::fmt::Display;
use std::io::Result;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct TreeFormatting {
    pub anchor_below: bool,
    pub down_facing_angle: char,
    pub down_facing_tee: char,
    pub vertical_line: char,
    pub horizontal_line: char,
    pub horizontal_space: char,
    pub horizontal_line_count: usize,
    pub right_facing_tee: char,
    pub right_facing_angle: char,
    pub label_space_char: char,
    pub label_space_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleTreeNode {
    label: String,
    children: Vec<SimpleTreeNode>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn write_tree(
    tree: &SimpleTreeNode,
    to_writer: &mut impl Write,
    before: Option<&str>,
) -> Result<()> {
    write_tree_with(tree, to_writer, &ascii_formatting(true), before)
}

#[inline]
pub fn write_tree_with(
    tree: &SimpleTreeNode,
    to_writer: &mut impl Write,
    format: &TreeFormatting,
    before: Option<&str>,
) -> Result<()> {
    write_tree_inner(
        tree,
        to_writer,
        format,
        before.unwrap_or_default().to_string(),
        Default::default(),
    )
}

#[inline]
pub fn to_string(tree: &SimpleTreeNode, before: Option<&str>) -> Result<String> {
    to_string_with(tree, &ascii_formatting(true), before)
}

#[inline]
pub fn to_string_with(
    tree: &SimpleTreeNode,
    format: &TreeFormatting,
    before: Option<&str>,
) -> Result<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    write_tree_with(tree, &mut buffer, format, before)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}

pub fn ascii_formatting(anchor_below: bool) -> TreeFormatting {
    TreeFormatting {
        anchor_below,
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

pub fn box_char_formatting(anchor_below: bool) -> TreeFormatting {
    TreeFormatting {
        anchor_below,
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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<String> for SimpleTreeNode {
    fn from(s: String) -> Self {
        Self {
            label: s,
            children: Default::default(),
        }
    }
}

impl From<&str> for SimpleTreeNode {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl SimpleTreeNode {
    pub fn new<L>(label: L) -> Self
    where
        L: Display,
    {
        Self {
            label: label.to_string(),
            children: Default::default(),
        }
    }

    pub fn with_children<L>(label: L, children: impl Iterator<Item = SimpleTreeNode>) -> Self
    where
        L: Display,
    {
        Self {
            label: label.to_string(),
            children: children.collect(),
        }
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn children(&self) -> impl Iterator<Item = &String> {
        self.children.iter().map(|c| c.label())
    }

    pub fn push<V>(&mut self, child_node: V)
    where
        V: Display,
    {
        self.children.push(SimpleTreeNode {
            label: child_node.to_string(),
            children: Default::default(),
        })
    }

    pub fn extend<V>(&mut self, children: impl Iterator<Item = V>)
    where
        V: Display + Sized,
    {
        self.children
            .extend(children.map(|child| child.to_string().into()))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_tree_inner(
    node: &SimpleTreeNode,
    w: &mut impl Write,
    format: &TreeFormatting,
    before: String,
    remaining_children_stack: Vec<usize>,
) -> Result<()> {
    // Write any requested prefix
    if !before.is_empty() {
        write!(w, "{}", before)?;
    }

    if !format.anchor_below && remaining_children_stack.is_empty() {
        write!(
            w,
            "{}{}",
            format.down_facing_angle,
            char_repeat(format.label_space_char, format.label_space_count)
        )?;
    }

    // Write the leading structures
    let stack_depth = remaining_children_stack.len();
    for (row, remaining_children) in remaining_children_stack.iter().enumerate() {
        write!(
            w,
            "{}",
            match (*remaining_children, row == (stack_depth - 1)) {
                (1, true) => angle(format, node.has_children()),
                (1, false) => just_space(format),
                (_, true) => tee(format, node.has_children()),
                (_, false) => bar_and_space(format),
            }
        )?;
    }

    // Write the node label, and any children (recursively)
    if node.has_children() {
        writeln!(w, "{}", node.label)?;
        let mut d = node.children.len();
        for child in &node.children {
            let mut new_child_stack = remaining_children_stack.clone();
            new_child_stack.push(d);
            d -= 1;
            write_tree_inner(child, w, format, before.clone(), new_child_stack)?;
        }
    } else {
        writeln!(w, "{}", node.label)?;
    }

    // All done :)
    Ok(())
}

fn just_space(format: &TreeFormatting) -> String {
    format!(
        "{}{}",
        format.horizontal_space,
        char_repeat(format.horizontal_space, format.horizontal_line_count),
    )
}

fn bar_and_space(format: &TreeFormatting) -> String {
    format!(
        "{}{}",
        format.vertical_line,
        char_repeat(format.horizontal_space, format.horizontal_line_count),
    )
}

fn tee(format: &TreeFormatting, has_children: bool) -> String {
    format!(
        "{}{}{}{}",
        format.right_facing_tee,
        char_repeat(format.horizontal_line, format.horizontal_line_count),
        if format.anchor_below {
            String::new()
        } else if has_children {
            format.down_facing_tee.to_string()
        } else {
            format.horizontal_line.to_string()
        },
        char_repeat(format.label_space_char, format.label_space_count)
    )
}

fn angle(format: &TreeFormatting, has_children: bool) -> String {
    format!(
        "{}{}{}{}",
        format.right_facing_angle,
        char_repeat(format.horizontal_line, format.horizontal_line_count),
        if format.anchor_below {
            String::new()
        } else if has_children {
            format.down_facing_tee.to_string()
        } else {
            format.horizontal_line.to_string()
        },
        char_repeat(format.label_space_char, format.label_space_count),
    )
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
        let node = SimpleTreeNode::new(String::from("hello"));
        assert_eq!(
            node,
            SimpleTreeNode {
                label: "hello".to_string(),
                children: vec![]
            }
        );
    }

    #[test]
    fn test_node_with_children() {
        let node =
            SimpleTreeNode::with_children(String::from("hello"), vec!["world".into()].into_iter());
        assert_eq!(
            node,
            SimpleTreeNode {
                label: "hello".to_string(),
                children: vec![SimpleTreeNode {
                    label: "world".to_string(),
                    children: vec![]
                }]
            }
        );
    }

    #[test]
    fn test_node_from_string() {
        let node: SimpleTreeNode = String::from("hello").into();
        assert_eq!(
            node,
            SimpleTreeNode {
                label: "hello".to_string(),
                children: vec![]
            }
        );
    }

    #[test]
    fn test_node_from_str() {
        let node: SimpleTreeNode = "hello".into();
        assert_eq!(
            node,
            SimpleTreeNode {
                label: "hello".to_string(),
                children: vec![]
            }
        );
    }
}
