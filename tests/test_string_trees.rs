use text_trees::*;

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

#[test]
fn test_ascii_below_tree() {
    let tree = make_tree();

    let result = to_string_with(&tree, &ascii_formatting(true), None);
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{}", result);
    assert_eq!(
        result,
        r#"root
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
"#
        .to_string()
    );
}

#[test]
fn test_box_char_below_tree() {
    let tree = make_tree();

    let result = to_string_with(&tree, &box_char_formatting(true), None);
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{}", result);
    assert_eq!(
        result,
        r#"root
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
"#
        .to_string()
    );
}

#[test]
fn test_ascii_side_tree() {
    let tree = make_tree();

    let result = to_string_with(&tree, &ascii_formatting(false), None);
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

    let result = to_string_with(&tree, &box_char_formatting(false), None);
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
        anchor_below: false,
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
    };

    let result = to_string_with(&tree, &format, Some(".. "));
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
