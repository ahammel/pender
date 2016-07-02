use std::collections::HashMap;

use event::Event;
use hash::Blake2;

/// A Pender database fragment.
/// 
/// Contains a set of Events ordered by parent relationships
/// (see `pender::event::Event`). An empty Fragment has no head.
///
/// ```
/// use pender::fragment::Fragment;
/// use pender::event::Event;
///
/// let mut frag = Fragment::new();
/// assert_eq!(frag.head, None);
///
/// let stuff = b"Stuff happened";
/// let root = Event::new(stuff, None);
/// frag.append(stuff);
/// assert_eq!(frag.head, Some(root));
///
/// let new_event = Event::new(b"More stuff happened", Some(root));
/// frag.append_event(new_event);
/// assert_eq!(frag.head, Some(new_event));
/// ```
#[derive(Clone, Debug, Default)]
pub struct Fragment<'a> {
    pub head: Option<Event<'a>>,
    pub events: HashMap<Blake2, Event<'a>>,
}

impl<'a> Fragment<'a> {
    pub fn new() -> Fragment<'a> { Default::default() }

    pub fn append(&mut self, fact: &'a [u8]) { 
        let head = self.head;
        self.append_event(Event::new(fact, head));
    }

    pub fn append_event (&mut self, event: Event<'a>) { 
        self.head = Some(event);
        self.events.insert(event.hash(), event);
    }
}
