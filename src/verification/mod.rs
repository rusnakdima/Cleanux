//! Verification engine re-exported from `dioxus-shared`.
//!
//! Canonical engine lives in `dioxus_shared::verify`; this shim re-exports
//! it verbatim so that local `use crate::verification::*` call-sites compile
//! without any path changes. App-specific extraction helpers are defined locally.

pub use dioxus_shared::verify::{
  dom, matcher, normalize, report, compare_html,
  CompareOptions, Report, Verdict,
};

pub use dioxus_shared::verify::normalize::Options as NormalizeOptions;

// App-specific: extract one `<section data-page="route">…</section>` from a template
// document, stripping template-only noise (`hidden` visibility class,
// `&nbsp;` placeholder used by the demo JS for an empty display line).
pub fn extract_page_section(template: &str, route: &str) -> Option<String> {
  // Try data-page first (template), then fall back to id (Dioxus SSR output)
  let marker_data = format!("<section data-page=\"{route}\"");
  let marker_id = format!("<section id=\"{route}\"");
  let start = template.find(&marker_data).or_else(|| template.find(&marker_id))?;
  let end_rel = template[start..].find("</section>")?;
  let end = start + end_rel + "</section>".len();
  Some(
    template[start..end]
      .replace(" class=\"hidden\"", "")
      .replace("&nbsp;", ""),
  )
}
