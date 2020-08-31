use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::{env, fs};
use text_trees::TreeNode;

const P_HOME: &str = "🏠";
const P_FOLDER: &str = "📁";
const P_FILE: &str = "📄";
const P_LINK: &str = "🔗";
const P_GONE: &str = "☠️";

struct FSEntry(PathBuf);

type FSTreeNode = TreeNode<FSEntry>;

fn main() {
    let fs_tree = make_dir_tree(PathBuf::from("."));
    fs_tree.write(&mut std::io::stdout()).unwrap();
}

fn make_dir_tree(path: PathBuf) -> FSTreeNode {
    let mut current_node = FSTreeNode::new(path.clone().into());
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let node = make_dir_tree(entry.path());
            current_node.push_node(node);
        }
    }
    current_node
}

impl From<PathBuf> for FSEntry {
    fn from(v: PathBuf) -> Self {
        Self(v)
    }
}

impl Display for FSEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.0.is_file() {
                let metadata = fs::symlink_metadata(&self.0).unwrap();
                let file_type = metadata.file_type();
                format!(
                    "{} {}",
                    if file_type.is_symlink() {
                        P_LINK
                    } else {
                        P_FILE
                    },
                    self.0.file_name().unwrap().to_string_lossy()
                )
            } else if self.0.is_dir() {
                let home = env::var("HOME").unwrap();
                let canonical_path = self.0.canonicalize().unwrap();
                format!(
                    "{} {}",
                    if canonical_path == PathBuf::from(home) {
                        P_HOME
                    } else {
                        P_FOLDER
                    },
                    match self.0.file_name() {
                        None => self.0.to_string_lossy(),
                        Some(path) => path.to_string_lossy(),
                    }
                )
            } else {
                format!(
                    "{} {}",
                    P_GONE,
                    self.0.file_name().unwrap().to_string_lossy()
                )
            }
        )
    }
}
