use failure::Error;
use std::ops::{Deref, DerefMut, Range};
use std::rc::Rc;
use std::str;

#[macro_use]
mod debug;

pub mod input;
pub use input::*;

pub mod cell;
pub use cell::*;

pub mod line;
pub use line::*;

pub mod screen;
pub use screen::*;

pub mod selection;
use selection::{SelectionCoordinate, SelectionRange};

pub mod hyperlink;
use hyperlink::Hyperlink;

pub mod terminal;
pub use terminal::*;

pub mod terminal_state;
pub use terminal_state::*;

/// Represents the index into screen.lines.  Index 0 is the top of
/// the scrollback (if any).  The index of the top of the visible screen
/// depends on the terminal dimensions and the scrollback size.
pub type PhysRowIndex = usize;

/// Represents an index into the visible portion of the screen.
/// Value 0 is the first visible row.  VisibleRowIndex needs to be
/// resolved into a PhysRowIndex to obtain an actual row.  It is not
/// valid to have a negative VisibleRowIndex value so this type logically
/// should be unsigned, however, having a different sign is helpful to
/// have the compiler catch accidental arithmetic performed between
/// PhysRowIndex and VisibleRowIndex.  We could define our own type with
/// its own Add and Sub operators, but then we'd not be able to iterate
/// over Ranges of these types without also laboriously implementing an
/// iterator Skip trait that is currently only in unstable rust.
pub type VisibleRowIndex = i64;

/// Like VisibleRowIndex above, but can index backwards into scrollback.
/// This is deliberately a differently sized signed type to catch
/// accidentally blending together the wrong types of indices.
/// This is explicitly 32-bit rather than 64-bit as it seems unreasonable
/// to want to scroll back or select more than ~2billion lines of scrollback.
pub type ScrollbackOrVisibleRowIndex = i32;

/// range.contains(), but that is unstable
pub fn in_range<T: PartialOrd>(value: T, range: &Range<T>) -> bool {
    value >= range.start && value < range.end
}

/// Position allows referring to an absolute visible row number
/// or a position relative to some existing row number (typically
/// where the cursor is located).  Both of the cases are represented
/// as signed numbers so that the math and error checking for out
/// of range values can be deferred to the point where we execute
/// the request.
#[derive(Debug)]
pub enum Position {
    Absolute(VisibleRowIndex),
    Relative(i64),
}

/// Describes the location of the cursor in the visible portion
/// of the screen.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct CursorPosition {
    pub x: usize,
    pub y: VisibleRowIndex,
}

pub mod color;
mod csi;
use self::csi::*;

/// The response we given when queries for device attributes.
/// This particular string says "we are a VT102".
/// TODO: Consider VT220 extended response which can advertise
/// certain feature sets.
pub const DEVICE_IDENT: &[u8] = b"\x1b[?6c";

#[allow(dead_code)]
pub const CSI: &[u8] = b"\x1b[";
#[allow(dead_code)]
pub const OSC: &[u8] = b"\x1b]";
#[allow(dead_code)]
pub const ST: &[u8] = b"\x1b\\";
#[allow(dead_code)]
pub const DCS: &[u8] = b"\x1bP";
