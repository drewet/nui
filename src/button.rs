//!
//!  button.rs
//!
//!  Created by Mitchell Nordine at 05:50PM on July 27, 2014.
//!
//!

use piston::{
    GameWindow,
};
use widget;
use widget::Widget;
use point::Point;
use std::default::Default;

/// Button type widget.
pub struct Button {
    widget_data: widget::Data,
    width: int,
    height: int,
}

impl Button {
    /// Constructor for Button widget.
    pub fn new(pos: Point<int>, width: int, height: int, event: ||) -> Button {
        Button {
            widget_data: widget::Data::new(pos),
            width: width,
            height: height,
        }
    }
}

impl Default for Button {
    /// Default constructor for Button widget.
    fn default() -> Button {
        Button {
            widget_data: Default::default(),
            width: 25,
            height: 25,
        }
    }
}

impl<W: GameWindow> Widget<W> for Button {
    impl_get_widget_data!(widget_data)
}

