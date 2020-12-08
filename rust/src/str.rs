//! `&str` utilities

pub fn split_once<'a>(s: &'a str, pat: &'a str) -> Option<(&'a str, &'a str)> {
    let mut split = s.splitn(2, pat);
    match (split.next(), split.next()) {
        (Some(s1), Some(s2)) => Some((s1, s2)),
        _ => None,
    }
}

pub fn rsplit_once<'a>(s: &'a str, pat: &'a str) -> Option<(&'a str, &'a str)> {
    let mut split = s.rsplitn(2, pat);
    match (split.next(), split.next()) {
        (Some(s1), Some(s2)) => Some((s1, s2)),
        _ => None,
    }
}
