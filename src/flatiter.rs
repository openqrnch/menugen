use crate::menuitem;

pub struct MenuIter<'a, C>
where
  C: Clone + Default
{
  lst: &'a Vec<menuitem::MenuItem<C>>,
  idx: usize
}

impl<'a, C> MenuIter<'a, C>
where
  C: Clone + Default
{
  pub fn new(cont: &'a Vec<menuitem::MenuItem<C>>) -> Self {
    MenuIter { lst: cont, idx: 0 }
  }
}

impl<'a, C> Iterator for MenuIter<'a, C>
where
  C: Clone + Default
{
  type Item = &'a menuitem::MenuItem<C>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.idx < self.lst.len() {
      let node = &self.lst[self.idx];
      self.idx += 1;
      Some(node)
    } else {
      None
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
