pub mod flatiter;
pub mod menu;
pub mod menuitem;
pub mod reciter;

pub mod iter {
  pub use crate::reciter::Event;
}

pub use menu::Builder as MenuBuilder;
pub use menuitem::Builder as MenuItemBuilder;

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
