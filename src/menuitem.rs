use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Representation of a menu item builder.
///
/// Instances of this object are added to the `Menu` builder.
#[derive(Default)]
pub struct Builder<C>
where
  C: Clone
{
  pub(crate) miid: String,
  pub(crate) title: String,
  pub(crate) parent: Option<String>,
  pub(crate) weight: isize,
  pub(crate) appctx: C
}

impl<C> Builder<C>
where
  C: Clone + Default
{
  pub fn new<M, T>(miid: M, title: T) -> Self
  where
    M: ToString,
    T: ToString
  {
    Builder {
      miid: miid.to_string(),
      title: title.to_string(),
      weight: 100,
      ..Default::default()
    }
  }
  pub fn new_ctx<M, T>(miid: M, title: T, appctx: C) -> Self
  where
    M: ToString,
    T: ToString
  {
    Builder {
      miid: miid.to_string(),
      title: title.to_string(),
      weight: 100,
      appctx,
      ..Default::default()
    }
  }

  pub fn parent<M: ToString>(&mut self, parent: M) -> &mut Self {
    self.parent = Some(parent.to_string());
    self
  }
  pub fn weight(&mut self, weight: isize) -> &mut Self {
    self.weight = weight;
    self
  }

  pub(crate) fn build(self) -> MenuItem<C> {
    MenuItem {
      miid: self.miid,
      title: self.title,
      weight: self.weight,
      children: Vec::new(),
      appctx: self.appctx
    }
  }
}

impl<C> Hash for Builder<C>
where
  C: Clone + Default
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.miid.hash(state);
  }
}


/// Representation of a single menu item.
#[derive(Default, Clone, Debug)]
pub struct MenuItem<C>
where
  C: Clone + Default
{
  miid: String,
  title: String,
  weight: isize,
  pub(crate) children: Vec<MenuItem<C>>,
  appctx: C
}


impl<C> MenuItem<C>
where
  C: Clone + Default
{
  /// Return a string reference to the menu item's internal identifier.
  pub fn id(&self) -> &str {
    &self.miid
  }

  /// Return a string reference to the menu item's title.
  pub fn title(&self) -> &str {
    &self.title
  }

  /// Return a reference to a Vec of all the child menu items.
  pub fn children(&self) -> &Vec<MenuItem<C>> {
    &self.children
  }

  pub fn is_parent(&self) -> bool {
    self.children.is_empty()
  }

  /// Return a reference to the application defined menu item context.
  pub fn appctx(&self) -> &C {
    &self.appctx
  }

  pub(crate) fn order_cmp(&self, other: &Self) -> Ordering {
    if self.weight == other.weight {
      return self.title.cmp(&other.title);
    }
    self.weight.cmp(&other.weight)
  }
}

impl<C> Hash for MenuItem<C>
where
  C: Clone + Default
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.miid.hash(state);
  }
}


#[cfg(test)]
mod tests {
  use super::Builder;
  use std::sync::Arc;

  #[test]
  fn sort_weight() {
    let mut bldr = Builder::<()>::new("test-1", "Second");
    bldr.weight(50);
    let mi1 = bldr.build();

    let mut bldr = Builder::new("test-2", "First");
    bldr.weight(10);
    let mi2 = bldr.build();

    let mut menuitems = vec![Arc::new(mi1), Arc::new(mi2)];

    assert_eq!(menuitems.len(), 2);
    assert_eq!(menuitems[0].title(), "Second");
    assert_eq!(menuitems[1].title(), "First");

    menuitems.sort_by(|a, b| a.order_cmp(b));

    assert_eq!(menuitems.len(), 2);
    assert_eq!(menuitems[0].title(), "First");
    assert_eq!(menuitems[1].title(), "Second");
  }

  #[test]
  fn sort_weight_title() {
    let bldr = Builder::<()>::new("test-1", "A menu item");
    let mi1 = bldr.build();

    let bldr = Builder::new("test-2", "B menu item");
    let mi2 = bldr.build();

    let mut menuitems = vec![Arc::new(mi2), Arc::new(mi1)];

    assert_eq!(menuitems.len(), 2);
    assert_eq!(menuitems[0].title(), "B menu item");
    assert_eq!(menuitems[1].title(), "A menu item");

    menuitems.sort_by(|a, b| a.order_cmp(b));

    assert_eq!(menuitems.len(), 2);
    assert_eq!(menuitems[0].title(), "A menu item");
    assert_eq!(menuitems[1].title(), "B menu item");
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
