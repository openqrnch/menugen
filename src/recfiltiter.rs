use crate::menu::Menu;
use crate::menuitem::MenuItem;

pub enum Event<'a, C>
where
  C: Clone + Default
{
  EnterScope,
  LeaveScope,
  MenuItem(&'a MenuItem<C>)
}

struct IterNode<'a, C>
where
  C: Clone + Default
{
  lst: &'a Vec<MenuItem<C>>,
  idx: usize,
  did_enter_scope: bool,
  did_leave_scope: bool
}

pub struct MenuIter<'a, C, F>
where
  C: Clone + Default,
  F: Fn(&MenuItem<C>) -> bool
{
  stack: Vec<IterNode<'a, C>>,
  myfilter: F
}

impl<'a, C, F> MenuIter<'a, C, F>
where
  C: Clone + Default,
  F: Fn(&MenuItem<C>) -> bool
{
  pub fn new(menu: &'a Menu<C>, p: F) -> Self {
    let mut iterstack = Vec::new();

    // Default to not scoping the root nodes
    iterstack.push(IterNode {
      lst: &menu.rootlst,
      idx: 0,
      did_enter_scope: true,
      did_leave_scope: true
    });

    MenuIter {
      stack: iterstack,
      myfilter: p
    }
  }

  /// Tell the iterator to scope the root items.  By default the iterator will
  /// not generate scope events for the root elements.
  ///
  /// # Constraints
  /// This setting must only be changed before iteration has started.
  pub fn root_scope(&mut self) -> &mut Self {
    if let Some(root) = self.stack.first_mut() {
      root.did_enter_scope = false;
      root.did_leave_scope = false;
    }
    self
  }
}


impl<'a, C, F> Iterator for MenuIter<'a, C, F>
where
  C: Clone + Default,
  F: Fn(&MenuItem<C>) -> bool
{
  type Item = Event<'a, C>;

  fn next(&mut self) -> Option<Self::Item> {
    while !self.stack.is_empty() {
      // last_mut()
      let mut it = self.stack.pop().unwrap();

      // If there's a filter defined then call it
      if it.idx < it.lst.len() {
        let mi = &it.lst[it.idx];

        // If the filter returns false then skip this entry
        if (self.myfilter)(mi) == false {
          it.idx += 1;
          self.stack.push(it);
          continue;
        }
      }

      // Let application know that iterator entered a new scope
      if it.did_enter_scope == false {
        it.did_enter_scope = true;
        self.stack.push(it);
        return Some(Event::EnterScope);
      }

      // Reached end of iterator
      if it.idx == it.lst.len() {
        // Before backing out make sure the application knows that a scope has
        // been left

        if it.did_enter_scope == true && it.did_leave_scope == false {
          it.did_leave_scope = true;
          self.stack.push(it);
          return Some(Event::LeaveScope);
        }

        continue;
      }


      if it.lst[it.idx].children.is_empty() {
        // Don't step into -- return next child in list
        let ret = Event::MenuItem(&it.lst[it.idx]);
        it.idx += 1;
        self.stack.push(it);
        return Some(ret);
      } else {
        let ret = Event::MenuItem(&it.lst[it.idx]);

        self.stack.push(IterNode {
          lst: &it.lst,
          idx: it.idx + 1,
          did_enter_scope: it.did_enter_scope,
          did_leave_scope: it.did_leave_scope
        });

        // Step into
        self.stack.push(IterNode {
          lst: &it.lst[it.idx].children,
          idx: 0,
          did_enter_scope: false,
          did_leave_scope: false
        });

        return Some(ret);
      }
    }
    None
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
