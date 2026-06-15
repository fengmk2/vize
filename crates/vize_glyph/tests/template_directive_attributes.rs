use vize_glyph::{FormatOptions, format_sfc, format_template};

#[test]
fn multiline_directive_attribute_value_is_indented_from_attribute_depth() {
    let source = r#"<span
  :class='[
rec.years.includes(y) && selectedYear === y
  ? "bg-accent border border-accent text-accent-ink"
  : rec.years.includes(y)
    ? "bg-ink border border-ink text-paper"
    : "border border-ink text-ink",
]'
  :title="y"
></span>"#;

    let options = FormatOptions::default();
    let first = format_template(source, &options).unwrap();
    let second = format_template(&first, &options).unwrap();

    assert_eq!(
        first.as_str(),
        r#"<span
  :class='[
    rec.years.includes(y) && selectedYear === y
      ? "bg-accent border border-accent text-accent-ink"
      : rec.years.includes(y)
        ? "bg-ink border border-ink text-paper"
        : "border border-ink text-ink",
  ]'
  :title="y"
></span>"#
    );
    assert_eq!(first, second);
}

#[test]
fn sfc_multiline_directive_attribute_keeps_template_indent() {
    let source = "<template>\n  <button\n    type=\"button\"\n    :class='sort === \"name-asc\" || sort === \"name-desc\"\n    ? \"bg-ink text-paper border-ink\"\n    : \"border-rule text-ink-2 hover:text-ink hover:border-ink\"'\n    @click=\"toggleNameSort\"\n  >\n    Name\n  </button>\n</template>\n";
    let options = FormatOptions::default();
    let first = format_sfc(source, &options).unwrap();
    let second = format_sfc(&first.code, &options).unwrap();

    assert_eq!(
        first.code.as_str(),
        "<template>\n  <button\n    type=\"button\"\n    :class='sort === \"name-asc\" || sort === \"name-desc\"\n      ? \"bg-ink text-paper border-ink\"\n      : \"border-rule text-ink-2 hover:text-ink hover:border-ink\"'\n    @click=\"toggleNameSort\"\n  >\n    Name\n  </button>\n</template>\n"
    );
    assert_eq!(first.code, second.code);
}
