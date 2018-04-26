use display_list::{DisplayAnnotationType, DisplayLine, DisplayMark, DisplaySnippetType};
use formatted_display_list::{FormattedDisplayLine, FormattedDisplayList};
use std::fmt;

impl fmt::Display for FormattedDisplayList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some((last, elements)) = self.body.split_last() {
            for line in elements.iter() {
                line.fmt(f)?;
                writeln!(f)?;
            }
            last.fmt(f)?;
        }
        Ok(())
    }
}

impl FormattedDisplayLine {
    pub fn format(dl: DisplayLine, lineno_width: usize, inline_marks_width: usize) -> Self {
        match dl {
            DisplayLine::Description {
                snippet_type,
                id,
                label,
            } => FormattedDisplayLine::Raw(format!(
                "{}[{}]: {}",
                Self::format_snippet_type(&snippet_type),
                id,
                label
            )),
            DisplayLine::Origin { path, row, col } => FormattedDisplayLine::Raw(format!(
                "{}--> {}:{}:{}",
                " ".repeat(lineno_width),
                path,
                row,
                col
            )),
            DisplayLine::EmptySource => FormattedDisplayLine::EmptySource {
                lineno: " ".repeat(lineno_width),
            },
            DisplayLine::Source {
                lineno,
                inline_marks,
                content,
                ..
            } => FormattedDisplayLine::Source {
                lineno: format!("{: >width$}", lineno, width = lineno_width),
                inline_marks: Self::format_inline_marks(&inline_marks, inline_marks_width),
                content,
            },
            DisplayLine::Annotation {
                inline_marks,
                range,
                label,
                annotation_type,
            } => FormattedDisplayLine::Annotation {
                lineno: " ".repeat(lineno_width),
                inline_marks: Self::format_inline_marks(&inline_marks, inline_marks_width),
                content: Self::format_annotation_content(range, &label, annotation_type),
            },
            DisplayLine::Fold => FormattedDisplayLine::Fold,
        }
    }

    fn format_snippet_type(snippet_type: &DisplaySnippetType) -> String {
        match snippet_type {
            DisplaySnippetType::Error => "error".to_string(),
            DisplaySnippetType::Warning => "warning".to_string(),
        }
    }

    fn format_inline_marks(inline_marks: &[DisplayMark], inline_marks_width: usize) -> String {
        format!(
            "{: >width$}",
            inline_marks
                .iter()
                .map(|mark| match mark {
                    DisplayMark::AnnotationThrough => "|",
                    DisplayMark::AnnotationStart => "/",
                })
                .collect::<Vec<&str>>()
                .join(""),
            width = inline_marks_width
        )
    }

    fn format_annotation_content(
        range: (usize, usize),
        label: &Option<String>,
        annotation_type: DisplayAnnotationType,
    ) -> String {
        let label = label.clone().map_or("".to_string(), |l| format!(" {}", l));
        match annotation_type {
            DisplayAnnotationType::Error => format!(
                "{}{}{}",
                " ".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::Warning => format!(
                "{}{}{}",
                " ".repeat(range.0),
                "-".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::MultilineStart => format!(
                "{}{}{}",
                "_".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::MultilineEnd => format!(
                "{}{}{}",
                "_".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
        }
    }
}

impl fmt::Display for FormattedDisplayLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormattedDisplayLine::EmptySource { lineno } => write!(f, "{} |", lineno),
            FormattedDisplayLine::Source {
                lineno,
                inline_marks,
                content,
            } => write!(f, "{} |{} {}", lineno, inline_marks, content),
            FormattedDisplayLine::Raw(body) => write!(f, "{}", body),
            FormattedDisplayLine::Annotation {
                lineno,
                inline_marks,
                content,
            } => write!(f, "{} |{}{}", lineno, inline_marks, content),
            FormattedDisplayLine::Fold => write!(f, "...  |"),
        }
    }
}