use menugen::{menu, menuitem};

#[derive(Clone, Default)]
struct Context {
  foo: bool
}

fn main() {
  let mut mb = menu::Builder::<Context>::new();
  let mib = menuitem::Builder::new("test-1", "Test A");

  mb.add(mib);

  let menu = mb.build();

  for mi in menu.iter_root() {
    println!("id: {}\ttitle: {}", mi.id(), mi.title());
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
