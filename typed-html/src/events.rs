//! Event handlers.

use crate::OutputType;
use htmlescape::encode_attribute;
use std::fmt::{Display, Error, Formatter};

/// Trait for event handlers.
pub trait EventHandler<T: OutputType + Send, E: Send> {
    /// Build a callback function from this event handler.
    ///
    /// Returns `None` is this event handler can't be used to build a callback
    /// function. This is usually the case if the event handler is a string
    /// intended for server side rendering.
    // fn build(self) -> Option<Box<FnMut(EventType) + 'static>>;

    fn attach(&mut self, target: &mut T::EventTarget) -> T::EventListenerHandle;

    /// Render this event handler as a string.
    ///
    /// Returns `None` if this event handler cannot be rendered. Normally, the
    /// only event handlers that can be rendered are string values intended for
    /// server side rendering.
    fn render(&self) -> Option<String>;
}

macro_rules! declare_events_struct {
    ($($name:ident,)*) => {
        pub struct Events<T> where T: Send {
            $(
                pub $name: Option<T>,
            )*
        }

        impl<T: Send> Events<T> {
            pub fn iter(&self) -> impl Iterator<Item = (&'static str, &T)> {
                let mut vec = Vec::new();
                $(
                    if let Some(ref value) = self.$name {
                        vec.push((stringify!($name), value));
                    }
                )*
                vec.into_iter()
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item = (&'static str, &mut T)> {
                let mut vec = Vec::new();
                $(
                    if let Some(ref mut value) = self.$name {
                        vec.push((stringify!($name), value));
                    }
                )*
                vec.into_iter()
            }
        }

        impl<T: 'static + Send> IntoIterator for Events<T> {
            type Item = (&'static str, T);
            type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

            fn into_iter(self) -> Self::IntoIter {
                let mut vec = Vec::new();
                $(
                    if let Some(value) = self.$name {
                        vec.push((stringify!($name), value));
                    }
                )*
                Box::new(vec.into_iter())
            }
        }

        impl<T: Send> Default for Events<T> {
            fn default() -> Self {
                Events {
                    $(
                        $name: None,
                    )*
                }
            }
        }

        impl<T: Display + Send> Display for Events<T> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $(
                    if let Some(ref value) = self.$name {
                        let attribute = encode_attribute(&value.to_string());
                        write!(f, " on{}=\"{}\"", stringify!($name), attribute)?;
                    }
                )*
                Ok(())
            }
        }
    }
}

declare_events_struct! {
    abort,
    autocomplete,
    autocompleteerror,
    blur,
    cancel,
    canplay,
    canplaythrough,
    change,
    click,
    close,
    contextmenu,
    cuechange,
    dblclick,
    drag,
    dragend,
    dragenter,
    dragexit,
    dragleave,
    dragover,
    dragstart,
    drop,
    durationchange,
    emptied,
    ended,
    error,
    focus,
    input,
    invalid,
    keydown,
    keypress,
    keyup,
    load,
    loadeddata,
    loadedmetadata,
    loadstart,
    mousedown,
    mouseenter,
    mouseleave,
    mousemove,
    mouseout,
    mouseover,
    mouseup,
    mousewheel,
    pause,
    play,
    playing,
    progress,
    ratechange,
    reset,
    resize,
    scroll,
    seeked,
    seeking,
    select,
    show,
    sort,
    stalled,
    submit,
    suspend,
    timeupdate,
    toggle,
    volumechange,
    waiting,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_events_iter() {
        let events: Events<&str> = Events::default();

        let mut iter = events.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_events_iter() {
        let events = Events::<&str> {
            abort: Some("abort"),
            waiting: Some("waiting"),
            ..Default::default()
        };

        let mut iter = events.iter();
        assert_eq!(iter.next(), Some(("abort", &"abort")));
        assert_eq!(iter.next(), Some(("waiting", &"waiting")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_events_iter_mut() {
        let mut events = Events::<&str> {
            abort: Some("abort"),
            waiting: Some("waiting"),
            ..Default::default()
        };

        let mut iter = events.iter_mut();
        assert_eq!(iter.next(), Some(("abort", &mut "abort")));
        assert_eq!(iter.next(), Some(("waiting", &mut "waiting")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_events_into_iter() {
        let events = Events::<&str> {
            abort: Some("abort"),
            waiting: Some("waiting"),
            ..Default::default()
        };

        let mut iter = events.into_iter();
        assert_eq!(iter.next(), Some(("abort", "abort")));
        assert_eq!(iter.next(), Some(("waiting", "waiting")));
        assert_eq!(iter.next(), None);
    }
}
