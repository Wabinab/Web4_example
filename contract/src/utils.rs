use crate::*;

pub(crate) fn common_page_template(
  page: &'static str,
) -> String {
    PRECOMPILED_BODY.replace("%CONTENT%", page)
}


