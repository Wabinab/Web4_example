use std::fs::File;
use std::io::prelude::*;

const HEAD_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/head.html");
// const BOOTSTRAP_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/bootstrap.html");
const CUSTOM_CSS_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/css.html");
const IMPORTMAP_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/importmap.html");
const JS_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/js.html");

const NAVBAR_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/navbar.html");
const LOGIN_CONTROLLER_BODY: &str = include_str!("/workspaces/Web4_example/design/controller/login_controller.html");

const FOOTER_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/footer.html");

const BODY_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/body.html");

fn main() -> std::io::Result<()> {
    let header: &str = &HEAD_BODY
      // .replace("%BOOTSTRAP%", BOOTSTRAP_BODY)
      .replace("%STYLE%", CUSTOM_CSS_BODY)
      .replace("%IMPORTMAP%", IMPORTMAP_BODY)
      .replace("%SCRIPT%", JS_BODY);

    let navbar: &str = &NAVBAR_BODY
      .replace("%LOGIN_CONTROLLER%", LOGIN_CONTROLLER_BODY);

    // Replace everything except %CONTENT% which we left to final renderer to replace. 
    let to_be_saved = BODY_BODY
                          .replace("%HEAD%", header)
                          .replace("%NAVBAR%", navbar)
                          .replace("%FOOTER%", FOOTER_BODY)
                          .to_owned();
  
    let mut file = File::create("/workspaces/Web4_example/design/precompile/precompile.html")?;
    file.write_all(to_be_saved.as_bytes())?;
    Ok(())
}
