use menugen::{menu, menuitem, reciter};


fn build_menu() -> menu::Menu<()> {
  let mut mb = menu::Builder::new();

  let mut mib =
    menuitem::Builder::new("test-sub-sub-menu", "Test sub-sub menu");
  mib.weight(100).parent("test-sub-menu");
  mb.add(mib);

  let mut mib = menuitem::Builder::new("sys-shutdown", "Shutdown");
  mib.weight(1000);
  mb.add(mib);

  let mut mib = menuitem::Builder::new("test-menu", "Test menu");
  mib.weight(100);
  mb.add(mib);

  let mut mib = menuitem::Builder::new("test-sub-menu", "Test sub menu");
  mib.weight(100).parent("test-menu");
  mb.add(mib);

  let mut mib = menuitem::Builder::new("test-menu-2", "Test menu too");
  mib.weight(100);
  mb.add(mib);

  let mut mib = menuitem::Builder::new("test-menu-3", "Aaaaargh!");
  mib.weight(100);
  mb.add(mib);

  let mut mib = menuitem::Builder::new("home", "Home");
  mib.weight(0);
  mb.add(mib);

  let mib = menuitem::Builder::new("foo", "Foo");
  mb.add(mib);

  let mut mib = menuitem::Builder::new("foo-sub-1", "Foo Sub");
  mib.parent("foo");
  mb.add(mib);

  let mut mib = menuitem::Builder::new("foo-sub-2", "Another Foo Sub");
  mib.parent("foo");
  mb.add(mib);

  mb.build()
}

fn main() {
  let menu = build_menu();

  let indent_str = String::from("  ");
  let mut indent = 0;
  for ev in menu.iter_hier() {
    match ev {
      reciter::Event::EnterScope => {
        println!("Enter scope");
        indent += 1;
      }
      reciter::Event::MenuItem(mi) => {
        println!(
          "{}id: {}\ttitle: {}",
          indent_str.repeat(indent),
          mi.id(),
          mi.title()
        );
      }
      reciter::Event::LeaveScope => {
        println!("Leave scope");
        indent -= 1;
      }
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
