use swc_core::common::{
    comments::Comment,
    pass::{AstNodePath, NodeRef},
};

pub fn is_comment_exist(comments: &Vec<Comment>, comment: &str) -> bool {
    return comments.iter().any(|&x| x.text.trim() == comment);
}
