use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::menuitem::{self, MenuItem};
use crate::{flatiter, reciter};

pub struct Builder<C>
where
  C: Clone + Default
{
  items: HashMap<String, menuitem::Builder<C>>
}

impl<C> Builder<C>
where
  C: Clone + Default
{
  pub fn new() -> Self {
    Builder {
      items: HashMap::new()
    }
  }

  /// Add a menu item to this menu.
  pub fn add(&mut self, mib: menuitem::Builder<C>) -> &mut Self {
    self.items.insert(mib.miid.to_string(), mib);
    self
  }

  pub fn build(self) -> Menu<C> {
    let mut rootitems: Vec<String> = Vec::new();
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();
    let mut menuitems: HashMap<String, Rc<RefCell<MenuItem<C>>>> =
      HashMap::new();

    // Iterate over all nodes to generate:
    // - a map of all parent identifiers to a list of their submenu identifiers
    // - a list of all menu item identifiers with no parents
    for (id, mib) in self.items {
      //println!("Processing miid '{}'", id);
      // If this menu item builder node has a parent, then add it to the
      // parent-to-children container.
      // Otherwise add it to the list of root menu items.
      if let Some(ref parent_id) = mib.parent {
        // Make sure parent exists in dictionary of all parents and their
        // subitems
        if !parents.contains_key(parent_id) {
          parents.insert(parent_id.to_string(), Vec::new());
        }

        // Get list of child node identifiers for this parent.
        // This is guaranteed to exist at this point.
        if let Some(idv) = parents.get_mut(parent_id) {
          // Add this (child menu item identifier) to the parent's list of
          // children.
          idv.push(id.to_string());
        }
      } else {
        // This item didn't specify a parent, which means that it is a root
        // menu item.
        rootitems.push(id.to_string());
      }

      // Build MenuItem object from this builder
      menuitems.insert(id.to_string(), Rc::new(RefCell::new(mib.build())));
    }

    // At this point:
    // - rootitems is a vector of all menu item identifiers that do not have
    //   any parents (i.e. are root items)
    // - parents is a hashmap of all parent identifiers mapped to a vector of
    //   all child menu item identifiers.
    // - menuitems is a hashmap of all menu item identifiers mapped to their
    //   MenuItem.

    // Create a stack of parent nodes.  These will be ordered so that child
    // nodes will be processed before their parents (which is important
    // when constructing the real tree later).

    let mut q = VecDeque::new();
    let mut stack = Vec::new();

    // push all root nodes that are parents onto the queue
    for id in &rootitems {
      if parents.contains_key(id) {
        q.push_back(id.clone());
      }
    }

    // As long as the queue is not empty:
    // - keep pulling nodes off it
    //   - push the node on to the stack
    //   - if the node contain children, then push then on to the queue
    while let Some(id) = q.pop_back() {
      // Push node on to the stack, because it has children
      stack.push(id.clone());

      // If node is a parent, then push its children that have children on to
      // the queue
      if let Some(child_ids) = parents.get(&id) {
        for child_id in child_ids {
          if parents.contains_key(child_id) {
            q.push_back(child_id.clone());
          }
        }
      }
    }


    // At this point "stack" is an stack of all parents.
    // Take the nodes off the stack one by one and clone the child nodes onto
    // their parents' child list.
    while let Some(parent_id) = stack.pop() {
      let parent = match menuitems.get(&parent_id) {
        Some(parent) => parent,
        None => continue
      };

      let child_ids = match parents.get(&parent_id) {
        Some(v) => v,
        None => continue
      };

      //println!("Adding children {:?} to parent {:?}", child_ids, parent);
      for child_id in child_ids {
        let child = match menuitems.get(child_id) {
          Some(child) => child,
          None => continue
        };
        //println!("Adding child {} to parent {}", child_id, parent_id);
        let mut parent = parent.borrow_mut();

        // Clone child node into child list.
        // The order is important here, which is why the stack is ordered the
        // way it is.
        parent.children.push(child.borrow().clone());
        //println!("{:?}", parent);
      }
      //println!("Parent2: {:?}\n", parent);

      // Sort child items
      let mut parent = parent.borrow_mut();
      parent.children.sort_by(|a, b| a.order_cmp(b));
    }

    //println!("\nParents: {:?}\n", parents);
    //println!("menuitems: {:?}\n", menuitems);


    let mut rootmis = Vec::new();

    // Put root nodes into root list
    for root_id in rootitems {
      if let Some(mi) = menuitems.get(&root_id) {
        rootmis.push(mi.borrow().clone());
      }
    }

    // sort root items
    rootmis.sort_by(|a, b| a.order_cmp(b));

    //println!("\nFinal: {:?}\n", rootmis);

    Menu { rootlst: rootmis }
  }
}


pub struct Menu<C>
where
  C: Clone + Default
{
  pub(crate) rootlst: Vec<MenuItem<C>>
}

impl<C> Menu<C>
where
  C: Clone + Default
{
  pub fn get_rootitems(&self) -> &Vec<MenuItem<C>> {
    &self.rootlst
  }

  pub fn iter_root(&self) -> flatiter::MenuIter<C> {
    flatiter::MenuIter::new(&self.rootlst)
  }

  pub fn iter_hier(&self) -> reciter::MenuIter<C> {
    reciter::MenuIter::new(self)
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
