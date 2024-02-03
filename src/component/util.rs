use std::borrow::Cow;

use ratatui::prelude::*;

pub(crate) fn split_message(message: &str, width: usize) -> Vec<Line> {
    let lines: Vec<Line> = message.split('\n').map(|line| Line::from(line)).collect();
    let mut result = Vec::new();
    for line in lines {
        if line.width() > width {
            result.extend(split_line(line, width));
        } else {
            result.push(line);
        }
    }
    result
}

pub(crate) fn split_line(line: Line, width: usize) -> Vec<Line> {
    let mut lhs = Line::default();
    lhs.alignment = line.alignment;
    let mut result = Vec::new();
    for mut span in line.spans {
        while lhs.width() + span.width() >= width {
            let (span_lhs, span_rhs) = split_span(span.clone(), width - lhs.width());
            lhs.spans.push(span_lhs);
            let alignment = lhs.alignment.clone();
            result.push(lhs);
            lhs = Line::default();
            lhs.alignment = alignment;
            span = span_rhs;
        }
        lhs.spans.push(span);
    }
    if lhs.width() > 0 {
        result.push(lhs);
    }
    result
}

pub(crate) fn split_span(span: Span, width: usize) -> (Span, Span) {
    let (lhs, rhs) = span.content.split_at(width);
    (
        Span {
            content: Cow::Owned(lhs.to_owned()),
            style: span.style,
        },
        Span {
            content: Cow::Owned(rhs.to_owned()),
            style: span.style,
        },
    )
}


mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_split_span() {
        let span = Span::from("Hello, world!".to_string());
        let (lhs, rhs) = split_span(span, 5);
        assert_eq!(lhs.content, "Hello");
        assert_eq!(rhs.content, ", world!");
    }

    #[test]
    fn test_split_line() {
        let line = Line::from("Hello, world!!!");
        let result = split_line(line, 5);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Line::from("Hello"));
        assert_eq!(result[1], Line::from(", wor"));
        assert_eq!(result[2], Line::from("ld!!!"));
    }

    #[test]
    fn test_split_message() {
        let message = "Hello, world!\nThis is a test message.";
        let result = split_message(message, 10);
        assert_eq!(result, vec![
            Line::from("Hello, wor"),
            Line::from("ld!"),
            Line::from("This is a "),
            Line::from("test messa"),
            Line::from("ge."),
        ]);
    }
}