#[derive(Debug)]
pub(crate) struct TokenStream(Vec<MathPrintToken>);

impl FromIterator<MathPrintToken> for TokenStream {
    fn from_iter<T: IntoIterator<Item = MathPrintToken>>(iter: T) -> Self {
        TokenStream(Vec::from_iter(iter))
    }
}

#[derive(Debug)]
pub(crate) enum MathPrintToken {
    String(String),
    AnnotationStart,
    AnnotationEnd,
}

impl IntoIterator for TokenStream {
    type Item = MathPrintToken;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<&str> for MathPrintToken {
    #[inline]
    fn from(value: &str) -> Self {
        MathPrintToken::String(value.to_string())
    }
}

impl From<String> for MathPrintToken {
    #[inline]
    fn from(value: String) -> Self {
        MathPrintToken::String(value)
    }
}

impl std::fmt::Display for TokenStream {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&print(self))?;
        Ok(())
    }
}

pub(crate) fn print(token_stream: &TokenStream) -> String {
    let mut math_line = String::new();
    let mut annotation_line = String::new();

    let mut has_seen_annotation = false;
    let mut is_annotation = false;
    for token in token_stream.0.iter() {
        match token {
            MathPrintToken::String(string) => {
                math_line.push_str(string);
                if is_annotation {
                    annotation_line.push_str(&"^".repeat(string.len()));
                } else {
                    annotation_line.push_str(&" ".repeat(string.len()));
                }
            }
            MathPrintToken::AnnotationStart => {
                is_annotation = true;
                has_seen_annotation = true;
            }
            MathPrintToken::AnnotationEnd => {
                is_annotation = false;
            }
        }
    }

    if has_seen_annotation {
        format!("{}\n{}", math_line, annotation_line.trim_end())
    } else {
        math_line
    }
}

#[macro_export]
macro_rules! token_iter {
    ($exp:literal) => {{
        use $crate::token_stream::MathPrintToken;
        let m: MathPrintToken = $exp.into();
        std::iter::once(m)
    }};
    ($exp:expr) => {{
        $exp.into_iter()
    }};
}

// TODO: most of these don't need to use iterators
#[macro_export]
macro_rules! tokens {
    ($exp:literal) => {{
        use $crate::token_stream::{MathPrintToken, TokenStream};
        TokenStream::from_iter(std::iter::once::<MathPrintToken>($exp.into()))
    }};
    ($exp:expr) => {{
        use $crate::token_stream::TokenStream;
        TokenStream::from_iter($exp)
    }};
    ($exp1:expr, $($exp:expr),*) => {{
        use $crate::token_stream::TokenStream;
        use $crate::token_iter;
        TokenStream::from_iter(token_iter!($exp1)$(.chain(token_iter!($exp)))*)
    }};
}
