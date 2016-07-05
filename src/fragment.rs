use std::collections::HashMap;

use event::Event;
use hash::Blake2;

/// A Pender database fragment.
/// 
/// Contains a set of Events ordered by parent relationships
/// (see `pender::event::Event`). An empty Fragment has no head.
///
/// # Examples
///
/// ```
/// use pender::event::Event;
/// use pender::fragment::{Chain, Fragment, Link};
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
///
/// let mut chain = frag.summarize("my-summary");
/// assert_eq!(chain.next_event(), Link::Event(new_event));
/// assert_eq!(chain.next_event(), Link::Event(root));
/// assert_eq!(chain.next_event(), Link::Terminus(None));
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

    pub fn summarize (self, name: &'a str) -> Chain<'a> {
        Chain::new(self, name)
    }
}

#[derive(Debug)]
pub struct Chain<'a> {
    fragment: Fragment<'a>,
    summary: &'a str,
    next: Option<Blake2>,
}

impl<'a> Chain<'a> {
    pub fn new(fragment: Fragment<'a>, summary: &'a str) -> Chain<'a> {
        let head = fragment.head;
        Chain {
            fragment: fragment,
            summary: summary,
            next: head.map(|e| e.hash())
        }
    }

    pub fn next_event(&mut self) -> Link<'a> {
        match self.next {
            None => Link::Terminus(None),
            Some(hash) => {
                if let Some(event) = self.fragment.events.get(&hash) {
                    self.next = event.parent();
                    Link::Event(*event)
                } else {
                    Link::Terminus(Some(hash))
                }
            }
        }
    }

    fn set_next(&mut self) {
        // TODO: implement
    }
}

#[derive(Debug, PartialEq)]
pub enum Link<'a> {
    Event(Event<'a>),
    Terminus(Option<Blake2>),
}
