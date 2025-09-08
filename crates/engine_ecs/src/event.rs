use std::marker::PhantomData;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
pub trait BufferedEvent: Send + Sync + 'static {}

pub struct EventId<E: BufferedEvent> {
    pub id: usize,
    pub(super) _marker: PhantomData<E>,
}


impl<E: BufferedEvent> Copy for EventId<E> {}

impl<E: BufferedEvent> Clone for EventId<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E: BufferedEvent> PartialEq for EventId<E> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<E: BufferedEvent> Eq for EventId<E> {}

impl<E: BufferedEvent> PartialOrd for EventId<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E: BufferedEvent> Ord for EventId<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<E: BufferedEvent> Hash for EventId<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state);
    }
}



pub struct EventInstance<E: BufferedEvent> {
    pub event_id: EventId<E>,
    pub event: E,
}

impl<E: BufferedEvent> Default for EventSequence<E> {
    fn default() -> Self {
        Self {
            events: Default::default(),
        }
    }
}

impl<E: BufferedEvent> Deref for EventSequence<E> {
    type Target = Vec<EventInstance<E>>;

    fn deref(&self) -> &Self::Target {
        &self.events
    }
}

impl<E: BufferedEvent> DerefMut for EventSequence<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.events
    }
}


pub(crate) struct EventSequence<E: BufferedEvent> {
    pub(crate) events: Vec<EventInstance<E>>,
}



pub struct Events<E: BufferedEvent> {
    pub(crate) events: EventSequence<E>,
}


impl<E: BufferedEvent> Default for Events<E> {
    fn default() -> Self {
        Self {
            events: Default::default(),
        }
    }
}

impl<E: BufferedEvent> Events<E> {

    pub fn write(&mut self, event: E) -> EventId<E> {
        let id = self.events.len();
        let event_id = EventId {
            id,
            _marker: PhantomData,
        };
        self.events.push(EventInstance { event_id, event });
        event_id
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn drain(&mut self) -> impl Iterator<Item = E> {
        self.events.drain(..).map(|i| i.event)
    }
}