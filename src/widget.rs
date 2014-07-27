//!
//!  widget.rs
//!
//!  Created by Mitchell Nordine at 05:38PM on July 27, 2014.
//!  
//!

use piston::{
    GameWindow,
    RenderArgs,
    UpdateArgs,
    KeyPressArgs,
    KeyReleaseArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseMoveArgs,
    MouseRelativeMoveArgs,
    MouseScrollArgs,
};
use point::Point;
use std::default::Default;

/// State of the widget.
pub enum DrawState {
    Normal,
    Highlighted,
    Clicked,
}

/// Essential data for widget types.
pub struct Data {
    pos: Point<int>,
    draw_state: DrawState,
    visible: bool,
}

impl Data {
    /// Basic constructor for Widget Data.
    pub fn new(pos: Point<int>) -> Data {
        Data {
            pos: pos,
            draw_state: Normal,
            visible: true,
        }
    }
}

impl Default for Data {
    /// Default constructor for Widget Data.
    fn default() -> Data {
        Data {
            pos: Default::default(),
            draw_state: Normal,
            visible: true,
        }
    }
}

/// The base trait for all UI Widget types.
/// A widget's children will always be drawn
/// AFTER the widget.
pub trait Widget<W: GameWindow> {

    /// Return all children widgets.
    fn get_children(&self) -> Vec<&Widget<W>> { Vec::new() }

    /// Return all children widgets.
    fn get_children_mut(&self) -> Vec<&mut Widget<W>> { Vec::new() }

    /// Return a reference to the widget data.
    fn get_widget_data(&self) -> &Data;

    /// Return a reference to the widget data.
    fn get_widget_data_mut(&mut self) -> &mut Data;

    /// Load the widget.
    fn load(&mut self, _w: &mut W) {}

    /// Render the widget.
    fn draw(&mut self, _w: &mut W, _args: &RenderArgs) {}

    /// Update the widget.
    fn update(&mut self, _w: &mut W, _args: &UpdateArgs) {}

    /// This can be overridden to handle key pressed events.
    fn key_press(&mut self, _w: &mut W, _args: &KeyPressArgs) {}

    /// This can be overridden to handle key released events.
    fn key_release(&mut self, _w: &mut W, _args: &KeyReleaseArgs) {}

    /// Pressed a mouse button.
    fn mouse_press(&mut self, _w: &mut W, _args: &MousePressArgs) {}

    /// Released a mouse button.
    fn mouse_release(&mut self, _w: &mut W, _args: &MouseReleaseArgs) {}

    /// Moved mouse cursor.
    fn mouse_move(&mut self, _w: &mut W, _args: &MouseMoveArgs) {}

    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _w: &mut W, _args: &MouseRelativeMoveArgs) {}

    /// Scrolled mouse.
    fn mouse_scroll(&mut self, _w: &mut W, _args: &MouseScrollArgs) {}

}

