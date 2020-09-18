use menugen::{menu, menuitem, reciter};

#[derive(Clone, Default)]
struct Context {
  enabled: bool
}

fn build_strvec<C>(it: reciter::MenuIter<C>) -> Vec<String>
where
  C: Clone + Default
{
  let mut out = Vec::new();
  let indent_str = String::from("  ");
  let mut indent = 0;
  for ev in it {
    match ev {
      reciter::Event::EnterScope => {
        indent += 1;
      }
      reciter::Event::MenuItem(mi) => {
        out.push(format!(
          "{}{}({})",
          indent_str.repeat(indent),
          mi.id(),
          mi.title()
        ));
      }
      reciter::Event::LeaveScope => {
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
fn filter_none_of_one() {
  let mut mb = menu::Builder::<Context>::new();
  let mib =
    menuitem::Builder::new_ctx("test-1", "Test A", Context { enabled: true });
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.itemfilter(|mi| mi.appctx().enabled == true);

  let expect = vec!["test-1(Test A)"];
  assert_eq!(verify(it, expect), true);
}


#[test]
fn filter_all_of_one() {
  let mut mb = menu::Builder::<Context>::new();
  let mib =
    menuitem::Builder::new_ctx("test-1", "Test A", Context { enabled: false });
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.itemfilter(|mi| mi.appctx().enabled == true);

  let expect = Vec::new();
  assert_eq!(verify(it, expect), true);
}


#[test]
fn filter_none_of_two() {
  let mut mb = menu::Builder::<Context>::new();
  let mib =
    menuitem::Builder::new_ctx("test-1", "Test A", Context { enabled: true });
  mb.add(mib);

  let mib =
    menuitem::Builder::new_ctx("test-2", "Test B", Context { enabled: true });
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.itemfilter(|mi| mi.appctx().enabled == true);

  let expect = vec!["test-1(Test A)", "test-2(Test B)"];
  assert_eq!(verify(it, expect), true);
}


#[test]
fn filter_all_of_two() {
  let mut mb = menu::Builder::<Context>::new();
  let mib =
    menuitem::Builder::new_ctx("test-1", "Test A", Context { enabled: false });
  mb.add(mib);

  let mib =
    menuitem::Builder::new_ctx("test-2", "Test B", Context { enabled: false });
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.itemfilter(|mi| mi.appctx().enabled == true);

  let expect = Vec::new();
  assert_eq!(verify(it, expect), true);
}


#[test]
fn filter_first_of_two() {
  let mut mb = menu::Builder::<Context>::new();
  let mib =
    menuitem::Builder::new_ctx("test-1", "Test A", Context { enabled: false });
  mb.add(mib);

  let mib =
    menuitem::Builder::new_ctx("test-2", "Test B", Context { enabled: true });
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.itemfilter(|mi| mi.appctx().enabled == true);

  let expect = vec!["test-2(Test B)"];
  assert_eq!(verify(it, expect), true);
}


#[test]
fn filter_second_of_two() {
  let mut mb = menu::Builder::<Context>::new();
  let mib =
    menuitem::Builder::new_ctx("test-1", "Test A", Context { enabled: true });
  mb.add(mib);

  let mib =
    menuitem::Builder::new_ctx("test-2", "Test B", Context { enabled: false });
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.itemfilter(|mi| mi.appctx().enabled == true);

  let expect = vec!["test-1(Test A)"];
  assert_eq!(verify(it, expect), true);
}


#[test]
fn filter_parent_two() {
  let mut mb = menu::Builder::<Context>::new();
  let mib =
    menuitem::Builder::new_ctx("test-1", "Test A", Context { enabled: true });
  mb.add(mib);

  let mib =
    menuitem::Builder::new_ctx("test-2", "Test B", Context { enabled: false });
  mb.add(mib);

  let menu = mb.build();

  let mut it = reciter::MenuIter::new(&menu);
  it.itemfilter(|mi| mi.appctx().enabled == true);

  let expect = vec!["test-1(Test A)"];
  assert_eq!(verify(it, expect), true);
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
