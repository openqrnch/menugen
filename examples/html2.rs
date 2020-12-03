use std::collections::HashSet;

use menugen::{menu, menuitem, recfiltiter};

#[derive(Clone, Default)]
struct Context {
  url: String,
  reqperms: HashSet<String>
}

fn build_strvec<F>(it: recfiltiter::MenuIter<Context, F>) -> Vec<String>
where
  Context: Clone + Default,
  F: Fn(&menuitem::MenuItem<Context>) -> bool
{
  let mut out = Vec::new();
  let indent_str = String::from("  ");
  let mut indent = 0;
  for ev in it {
    match ev {
      recfiltiter::Event::EnterScope => {
        out.push(format!("{}<ul>", indent_str.repeat(indent)));
        indent += 1;
      }
      recfiltiter::Event::MenuItem(mi) => {
        out.push(format!(
          r#"{}<li id="{}"><a href="{}">{}</a></li>"#,
          indent_str.repeat(indent),
          mi.id(),
          mi.appctx().url,
          mi.title()
        ));
      }
      recfiltiter::Event::LeaveScope => {
        indent -= 1;
        out.push(format!("{}</ul>", indent_str.repeat(indent)));
      }
    }
  }
  out
}

fn main() {
  let mut mb = menu::Builder::<Context>::new();

  let mut perms = HashSet::new();
  perms.insert("adm".to_owned());
  let mib = menuitem::Builder::new_ctx(
    "lib",
    "Library",
    Context {
      url: "https://library.org/".to_string(),
      reqperms: perms
    }
  );
  mb.add(mib);

  let mut perms = HashSet::new();
  perms.insert("adm".to_owned());
  let mut mib = menuitem::Builder::new_ctx(
    "lib-kb",
    "Knowledge Base",
    Context {
      url: "https://library.org/kb".to_string(),
      reqperms: perms
    }
  );
  mib.parent("lib");
  mb.add(mib);

  let mut perms = HashSet::new();
  perms.insert("adm".to_owned());
  let mut mib = menuitem::Builder::new_ctx(
    "lib-kb-arch",
    "Archive",
    Context {
      url: "https://library.org/kb/archive".to_string(),
      reqperms: perms
    }
  );
  mib.parent("lib-kb");
  mb.add(mib);

  let mut perms = HashSet::new();
  perms.insert("adm".to_owned());
  let mut mib = menuitem::Builder::new_ctx(
    "lib-secret",
    "Secret",
    Context {
      url: "https://library.org/secret".to_string(),
      reqperms: perms
    }
  );
  mib.parent("lib");
  mb.add(mib);

  let mut perms = HashSet::new();
  perms.insert("adm".to_owned());
  let mut mib = menuitem::Builder::new_ctx(
    "lib-secret-secret",
    "Very Secret",
    Context {
      url: "https://library.org/secret/very".to_string(),
      reqperms: perms
    }
  );
  mib.parent("lib-secret");
  mb.add(mib);

  let menu = mb.build();

  let mut account_perms = HashSet::new();
  account_perms.insert("adm".to_owned());
  let mut rit = menu.filtiter_hier(|mi| {
    let isect = account_perms.intersection(&mi.appctx().reqperms);
    let ret: HashSet<_> = isect.collect();
    !ret.is_empty()
  });
  rit.root_scope();

  let html = build_strvec(rit);

  for s in html {
    println!("{}", s);
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
