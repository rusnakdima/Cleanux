//! Verification engine stub for Cleanux.
//!
//! The full verification engine is in dioxus_shared. This stub provides
//! minimal local functionality.

/// App-specific: extract one `<section data-page="route">…</section>` from a template
/// document, stripping template-only noise (`hidden` visibility class,
/// `&nbsp;` placeholder used by the demo JS for an empty display line).
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

// Re-exports for compatibility - stub types
#[derive(Debug, Clone)]
pub struct Report;
#[derive(Debug, Clone)]
pub struct Verdict;
#[derive(Debug, Clone)]
pub struct CompareOptions;
