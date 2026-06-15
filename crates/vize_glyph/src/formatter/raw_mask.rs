/// Per-line "this line is inside a whitespace-significant block" mask.
///
/// Lines inside `<pre>`, `<textarea>`, `v-pre`, multi-line comments, and
/// literal multi-line attribute values are raw. Directive expression
/// continuation lines are formatter output, so they still get SFC indentation.
pub(super) fn compute_raw_line_mask(lines: &[&[u8]]) -> Vec<bool> {
    let mut mask = vec![false; lines.len()];
    let mut depth_stack: Vec<&'static str> = Vec::new();
    let mut in_tag = false;
    let mut open_quote: Option<(u8, bool)> = None;
    let mut pending_raw_tag: Option<&'static str> = None;
    let mut in_comment = false;
    const TAGS: [(&str, &str, &str); 2] = [
        ("pre", "<pre", "</pre>"),
        ("textarea", "<textarea", "</textarea>"),
    ];

    for (i, line) in lines.iter().enumerate() {
        if !depth_stack.is_empty() || open_quote.is_some_and(|(_, raw)| raw) || in_comment {
            mask[i] = true;
        }

        let bytes = line;
        let mut cursor = 0;
        while cursor < bytes.len() {
            if in_comment {
                if bytes[cursor..].starts_with(b"-->") {
                    in_comment = false;
                    cursor += 3;
                } else {
                    cursor += 1;
                }
                continue;
            }
            if let Some((quote, _)) = open_quote {
                if bytes[cursor] == quote {
                    open_quote = None;
                }
                cursor += 1;
                continue;
            }
            if in_tag {
                match bytes[cursor] {
                    b'"' | b'\'' => {
                        open_quote = Some((bytes[cursor], literal_attr_quote(bytes, cursor)));
                    }
                    b'>' => {
                        in_tag = false;
                        if let Some(tag) = pending_raw_tag.take() {
                            depth_stack.push(tag);
                        }
                    }
                    _ => {}
                }
                cursor += 1;
                continue;
            }
            if bytes[cursor] != b'<' {
                cursor += 1;
                continue;
            }
            if bytes[cursor..].starts_with(b"<!--") {
                in_comment = true;
                cursor += 4;
                continue;
            }

            let mut matched = false;
            for (tag, open_needle, close_needle) in &TAGS {
                if starts_with_ascii_ci(&bytes[cursor..], close_needle.as_bytes()) {
                    if let Some(idx) = depth_stack.iter().rposition(|t| t == tag) {
                        depth_stack.remove(idx);
                    }
                    cursor += close_needle.len();
                    matched = true;
                    break;
                }
                if starts_with_ascii_ci(&bytes[cursor..], open_needle.as_bytes())
                    && bytes
                        .get(cursor + open_needle.len())
                        .copied()
                        .is_none_or(|after| matches!(after, b'>' | b' ' | b'\t' | b'\r' | b'/'))
                {
                    pending_raw_tag = Some(tag);
                    in_tag = true;
                    cursor += open_needle.len();
                    matched = true;
                    break;
                }
            }
            if matched {
                continue;
            }
            if depth_stack.is_empty()
                && let Some(after) = bytes.get(cursor + 1).copied()
                && (after.is_ascii_alphabetic() || after == b'/')
            {
                in_tag = true;
            }
            cursor += 1;
        }
    }
    mask
}

fn literal_attr_quote(line: &[u8], quote_pos: usize) -> bool {
    attr_name_before_quote(line, quote_pos).is_none_or(|name| !directive_expr_attr(name))
}

fn attr_name_before_quote(line: &[u8], quote_pos: usize) -> Option<&[u8]> {
    let mut pos = quote_pos;
    while pos > 0 && matches!(line[pos - 1], b' ' | b'\t') {
        pos -= 1;
    }
    if pos == 0 || line[pos - 1] != b'=' {
        return None;
    }
    pos -= 1;
    while pos > 0 && matches!(line[pos - 1], b' ' | b'\t') {
        pos -= 1;
    }
    let end = pos;
    while pos > 0
        && !matches!(
            line[pos - 1],
            b' ' | b'\t' | b'\r' | b'\n' | b'<' | b'>' | b'/'
        )
    {
        pos -= 1;
    }
    (pos < end).then_some(&line[pos..end])
}

fn directive_expr_attr(name: &[u8]) -> bool {
    name.starts_with(b":")
        || name.starts_with(b"@")
        || name.starts_with(b"v-if")
        || name.starts_with(b"v-else-if")
        || name.starts_with(b"v-show")
        || name.starts_with(b"v-for")
        || name.starts_with(b"v-model")
        || name.starts_with(b"v-bind")
        || name.starts_with(b"v-on")
        || name == b"v-html"
        || name == b"v-text"
}

fn starts_with_ascii_ci(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.len() >= needle.len()
        && haystack[..needle.len()]
            .iter()
            .zip(needle.iter())
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
}
