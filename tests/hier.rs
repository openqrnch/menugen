use menugen::reciter::Event;
use menugen::{menu, menuitem, reciter};

fn build_strvec<C>(it: reciter::MenuIter<C>) -> Vec<String>
where
  C: Clone + Default
{
  let mut out = Vec::new();
  let indent_str = String::from("  ");
  let mut indent = 0;
  for ev in it {
    match ev {
      Event::EnterScope => {
        indent += 1;
      }
      Event::MenuItem(mi) => {
        out.push(format!(
          "{}{}({})",
          indent_str.repeat(indent),
          mi.id(),
          mi.title()
        ));
      }
      Event::LeaveScope => {
        indent -= 1;
      }
    }
  }
  out
}

fn verify<C>(it: reciter::MenuIter<C>, expect: Vec<&str>) -> bool
where
  C: Clone + Default
{
  let res = build_strvec(it);

  assert_eq!(res.len(), expect.len());
  for i in 0..(expect.len()) {
    assert_eq!(res[i], expect[i]);
  }

  true
}


#[test]
fn simple_no_root_scope() {
  let mut mb = menu::Builder::<()>::new();
  let mib = menuitem::Builder::new("test-1", "Test A");
  mb.add(mib);

  let menu = mb.build();

  let it = reciter::MenuIter::new(&menu);

  let expect = vec!["test-1(Test A)"];
  assert_eq!(verify(it, expect), true);
}


#[test]
fn simple_root_scope() {
  let mut mb = menu::Builder::<()>::new();
  let mib = menuitem::Builder::new("test-1", "Test A");
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.root_scope();

  let expect = vec!["  test-1(Test A)"];
  assert_eq!(verify(it, expect), true);
}


#[test]
fn sub_root_no_scope() {
  let mut mb = menu::Builder::<()>::new();
  let mib = menuitem::Builder::new("test-1", "Test A");
  mb.add(mib);

  let mut mib = menuitem::Builder::new("test-1-1", "Test A Sub");
  mib.parent("test-1");
  mb.add(mib);

  let menu = mb.build();

  let it = reciter::MenuIter::new(&menu);

  let expect = vec!["test-1(Test A)", "  test-1-1(Test A Sub)"];
  assert_eq!(verify(it, expect), true);
}


#[test]
fn sub_root_scope() {
  let mut mb = menu::Builder::<()>::new();
  let mib = menuitem::Builder::new("test-1", "Test A");
  mb.add(mib);

  let mut mib = menuitem::Builder::new("test-1-1", "Test A Sub");
  mib.parent("test-1");
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.root_scope();

  let expect = vec!["  test-1(Test A)", "    test-1-1(Test A Sub)"];
  assert_eq!(verify(it, expect), true);
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
