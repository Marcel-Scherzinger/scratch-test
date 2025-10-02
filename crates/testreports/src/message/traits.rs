use std::hash::Hash;

use crate::Message;

impl<Level> Clone for Message<Level> {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            msg: self.msg.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<Level> Hash for Message<Level> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind().hash(state);
        self.msg().hash(state);
    }
}

impl<Level> PartialEq for Message<Level> {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind() && self.msg() == other.msg()
    }
}

impl<Level> Eq for Message<Level> {}

impl<Level> Ord for Message<Level> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind()
            .cmp(other.kind())
            .then(self.msg().cmp(other.msg()))
    }
}

impl<Level> PartialOrd for Message<Level> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
