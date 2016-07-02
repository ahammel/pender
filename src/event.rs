use hash::Hash;

/// A wrapper that imposes partial ordering on chunks of binary data (here
/// called "Facts").
///
/// # Example
/// ```
/// use pender::event::Event;
///
/// let grandma = Event::new(b"Aphrodite", None);
/// let dad = Event::new(b"Greg", Some(grandma));
/// let child = Event::new(b"Fitzroy", Some(dad));
///
/// assert_eq!(grandma.parent(), None);
/// assert_eq!(dad.parent(), Some(grandma.hash()));
/// assert_eq!(child.parent(), Some(dad.hash()));
/// ```
#[derive(Copy, Clone)]
pub enum Event<'a> {
    Root { fact: &'a [u8] },
    Node { fact: &'a [u8], parent_hash: Hash },
}

impl<'a> Event<'a> {
    /// Event constructor.
    ///
    /// Pass `Some(Event)` to make a Node event, or `None` for a Root.
    ///
    /// ```
    /// use pender::event::Event;
    ///
    /// let root = Event::new(b"potato", None);
    /// let node = Event::new(b"leaf", Some(root));
    /// assert!(root.is_root());
    /// assert!(!node.is_root());
    /// ```
    pub fn new(fact: &'a [u8], parent: Option<Event>) -> Event<'a> {
        match parent {
            None =>
                Event::Root { fact: fact },
            Some(event) =>
                Event::Node { fact: fact, parent_hash: event.hash() },
        }
    }

    /// Blake2 hash of an Event.
    ///
    /// In the case of a Node event, the hash is the same as that of the Fact.
    /// For Nodes, the parent's hash is appended to the Fact, and we take the
    /// Blake2 hash of that.
    pub fn hash(self) -> Hash {
        match self {
            Event::Root { fact } => {
                Hash::new(fact)
            },
            Event::Node { fact, parent_hash } => {
                let mut tmp = Vec::new();
                tmp.extend(fact.iter().cloned());
                tmp.extend(parent_hash.bytes.iter().cloned());
                Hash::new(&tmp)
            },
        }
    }

    /// Return the hash value of the parent Event, if any.
    pub fn parent(self) -> Option<Hash> {
        match self {
            Event::Root { .. } => None,
            Event::Node { parent_hash, .. } => Some(parent_hash),
        }
    }

    /// True if the Event is Root, else false.
    pub fn is_root(self) -> bool {
        match self {
            Event::Root {..} => true,
            Event::Node {..} => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Event;

    #[test]
    fn hash_root_self_equal() {
        let root_a = Event::new(b"foo", None);
        let root_b = Event::new(b"foo", None);
        assert_eq!(root_a.hash(), root_b.hash())
    }

    #[test]
    fn hash_root_unequal_when_facts_unequal() {
        let root_a = Event::new(b"foo", None);
        let root_b = Event::new(b"boo", None);
        assert!(root_a.hash() != root_b.hash())
    }

    #[test]
    fn hash_of_root_not_equal_to_hash_of_descendent() {
        let root = Event::new(b"foo", None);
        let descendent = Event::new(b"foo", Some(root));
        debug_assert!(root.hash() != descendent.hash());
    }

    #[test]
    fn hash_of_descendents_equal_when_events_and_parents_equal() {
        let root = Event::new(b"foo", None);
        let desc_a = Event::new(b"foo", Some(root));
        let desc_b = Event::new(b"foo", Some(root));
        assert_eq!(desc_a.hash(), desc_b.hash());

    }

    #[test]
    fn hash_of_descendents_not_equal_when_events_are_not_equal() {
        let root = Event::new(b"foo", None);
        let desc_a = Event::new(b"foo", Some(root));
        let desc_b = Event::new(b"bar", Some(root));
        assert!(desc_a.hash() != desc_b.hash());
    }

    #[test]
    fn hash_of_descendents_not_equal_when_parents_are_not_equal() {
        let root_a = Event::new(b"foo", None);
        let root_b = Event::new(b"bar", None);
        let desc_a = Event::new(b"foo", Some(root_a));
        let desc_b = Event::new(b"foo", Some(root_b));
        assert!(desc_a.hash() != desc_b.hash());
    }
}
