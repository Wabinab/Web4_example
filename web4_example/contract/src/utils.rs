use crate::*;

pub(crate) fn common_page_template(page: &'static str) -> String {
    // Not the most cpu efficient, but oh well, we can 
    // refactor in the future. 
    let header: &str = &HEAD_BODY
        .replace("%STYLE%", CSS_BODY)
        .replace("%IMPORTMAP%", IMPORTMAP_BODY)
        .replace("%SCRIPT%", JS_BODY);

    let navbar: &str = &NAVBAR_BODY
        .replace("%LOGIN_CONTROLLER%", LOGIN_CONTROLLER_BODY);


    page
      .replace("%HEAD%", header)
      .replace("%NAVBAR%", navbar)
      .replace("%FOOTER%", FOOTER_BODY)
      .to_owned()
}