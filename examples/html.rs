use menugen::{menu, menuitem, reciter};

#[derive(Clone, Default)]
struct Context {
  url: String,
  enabled: bool
}

fn build_strvec(it: reciter::MenuIter<Context>) -> Vec<String>
where
  Context: Clone + Default
{
  let mut out = Vec::new();
  let indent_str = String::from("  ");
  let mut indent = 0;
  for ev in it {
    match ev {
      reciter::Event::EnterScope => {
        out.push(format!("{}<ul>", indent_str.repeat(indent)));
        indent += 1;
      }
      reciter::Event::MenuItem(mi) => {
        out.push(format!(
          r#"{}<li id="{}"><a href="{}">{}</a></li>"#,
          indent_str.repeat(indent),
          mi.id(),
          mi.appctx().url,
          mi.title()
        ));
      }
      reciter::Event::LeaveScope => {
        indent -= 1;
        out.push(format!("{}</ul>", indent_str.repeat(indent)));
      }
    }
  }
  out
}

fn main() {
  let mut mb = menu::Builder::<Context>::new();

  let mib = menuitem::Builder::new_ctx(
    "lib",
    "Library",
    Context {
      url: "https://library.org/".to_string(),
      enabled: true
    }
  );
  mb.add(mib);

  let mut mib = menuitem::Builder::new_ctx(
    "lib-kb",
    "Knowledge Base",
    Context {
      url: "https://library.org/kb".to_string(),
      enabled: true
    }
  );
  mib.parent("lib");
  mb.add(mib);

  let mut mib = menuitem::Builder::new_ctx(
    "lib-kb-arch",
    "Archive",
    Context {
      url: "https://library.org/kb/archive".to_string(),
      enabled: true
    }
  );
  mib.parent("lib-kb");
  mb.add(mib);

  let mut mib = menuitem::Builder::new_ctx(
    "lib-secret",
    "Secret",
    Context {
      url: "https://library.org/secret".to_string(),
      enabled: false
    }
  );
  mib.parent("lib");
  mb.add(mib);

  let mut mib = menuitem::Builder::new_ctx(
    "lib-secret-secret",
    "Very Secret",
    Context {
      url: "https://library.org/secret/very".to_string(),
      enabled: true // should be hidden because parent is hidden
    }
  );
  mib.parent("lib-secret");
  mb.add(mib);

  let menu = mb.build();

  let mut rit = menu.iter_hier();
  rit.root_scope();
  rit.itemfilter(|mi| mi.appctx().enabled == true);

  let html = build_strvec(rit);

  for s in html {
    println!("{}", s);
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
