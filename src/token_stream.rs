pub(crate) struct TokenStream(Vec<MathPrintToken>);

impl FromIterator<MathPrintToken> for TokenStream {
    fn from_iter<T: IntoIterator<Item = MathPrintToken>>(iter: T) -> Self {
        TokenStream(Vec::from_iter(iter))
    }
}

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

impl From<&MathPrintToken> for String {
    #[inline]
    fn from(value: &MathPrintToken) -> String {
        match value {
            MathPrintToken::String(s) => s.to_owned(),
            MathPrintToken::AnnotationStart => "".to_owned(),
            MathPrintToken::AnnotationEnd => "".to_owned(),
        }
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
    token_stream
        .0
        .iter()
        .map(|t| -> String { t.into() })
        .collect()
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
