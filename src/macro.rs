//!
//!  macro.rs
//!
//!  Created by Mitchell Nordine at 07:02PM on July 27, 2014.
//!
//!

/// Simplification of the `get_widget_data` methods from the Widget trait.
macro_rules! impl_get_widget_data(
    ($data:ident) => (
        /// Return a reference to the widget data.
        fn get_widget_data(&self) -> &::widget::Data { &self.$data }
        /// Return a reference to the widget data.
        fn get_widget_data_mut(&mut self) -> &mut ::widget::Data { &mut self.$data }
    )
)


