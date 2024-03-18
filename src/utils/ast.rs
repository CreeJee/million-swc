use swc_core::common::comments::Comment;

pub fn is_comment_exist(comments: &Vec<Comment>, comment: &str) -> bool {
    return comments.iter().any(|x| x.text.trim() == comment);
}
