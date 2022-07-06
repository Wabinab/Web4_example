use crate::*;

pub(crate) fn common_page_template(
  page: &'static str,
) -> String {
    PRECOMPILED_BODY.replace("%CONTENT%", page)
}


pub(crate) fn card_and_paginate(
  page: &'static str,
) -> String {
    common_page_template(page)
        .replace("%PAGINATION%", PAGINATION_BODY)
        .replace("%CONSTRUCT_CARD%", CONSTRUCT_CARD_BODY)
}