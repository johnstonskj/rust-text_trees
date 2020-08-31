use text_trees::*;

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

#[test]
fn test_ascii_below_tree() {
    let tree = make_tree();

    let result = tree.to_string_with_format(&TreeFormatting::dir_tree(FormatCharacters::ascii()));
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{}", result);
    assert_eq!(
        result,
        r#"root
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
"#
        .to_string()
    );
}

#[test]
fn test_box_char_below_tree() {
    let tree = make_tree();

    let result =
        tree.to_string_with_format(&TreeFormatting::dir_tree(FormatCharacters::box_chars()));
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{}", result);
    assert_eq!(
        result,
        r#"root
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
"#
        .to_string()
    );
}

#[test]
fn test_ascii_side_tree() {
    let tree = make_tree();

    let result =
        tree.to_string_with_format(&TreeFormatting::dir_tree_left(FormatCharacters::ascii()));
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{}", result);
    assert_eq!(
        result,
        r#"+ root
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
"#
        .to_string()
    );
}

#[test]
fn test_box_char_side_tree() {
    let tree = make_tree();

    let result =
        tree.to_string_with_format(&TreeFormatting::dir_tree_left(FormatCharacters::box_chars()));
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{}", result);
    assert_eq!(
        result,
        r#"┌ root
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
"#
        .to_string()
    );
}

#[test]
fn test_spacing_in_tree() {
    let tree = make_tree();

    let format = TreeFormatting {
        prefix_str: Some(".. ".to_string()),
        orientation: TreeOrientation::TopDown,
        anchor: AnchorPosition::Left,
        chars: FormatCharacters {
            down_facing_angle: '┌',
            down_facing_tee: '┬',
            vertical_line: '│',
            horizontal_line: '─',
            horizontal_space: '#',
            horizontal_line_count: 5,
            right_facing_tee: '├',
            right_facing_angle: '└',
            label_space_char: '.',
            label_space_count: 2,
        },
    };

    let result = tree.to_string_with_format(&format);
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{}", result);
    assert_eq!(
        result,
        r#".. ┌..root
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
"#
        .to_string()
    );
}
