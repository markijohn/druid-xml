///! Original source code : https://github.com/RazrFalcon/simplecss/blob/master/src/selector.rs

use std::fmt;

/// A position in text.
///
/// Position indicates a row/line and a column in the original text. Starting from 1:1.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct TextPos {
    pub row: u32,
    pub col: u32,
}

impl TextPos {
    /// Constructs a new `TextPos`.
    ///
    /// Should not be invoked manually, but rather via `Stream::gen_text_pos`.
    pub fn new(row: u32, col: u32) -> TextPos {
        TextPos { row, col }
    }
}

impl fmt::Display for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}

/// A list of possible errors.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Error {
    /// The steam ended earlier than we expected.
    ///
    /// Should only appear on invalid input data.
    UnexpectedEndOfStream,

    /// An invalid ident.
    InvalidIdent(TextPos),

    /// An unclosed comment.
    InvalidComment(TextPos),

    /// An invalid declaration value.
    InvalidValue(TextPos),

    /// An invalid byte.
    #[allow(missing_docs)]
    InvalidByte { expected: u8, actual: u8, pos: TextPos },

    /// A missing selector.
    SelectorMissing,

    /// An unexpected selector.
    UnexpectedSelector,

    /// An unexpected combinator.
    UnexpectedCombinator,

    /// An invalid or unsupported attribute selector.
    InvalidAttributeSelector,

    /// An invalid language pseudo-class.
    InvalidLanguagePseudoClass,
}


trait CssCharExt {
    fn is_name_start(&self) -> bool;
    fn is_name_char(&self) -> bool;
    fn is_non_ascii(&self) -> bool;
    fn is_escape(&self) -> bool;
}

impl CssCharExt for char {
    #[inline]
    fn is_name_start(&self) -> bool {
        match *self {
            '_' | 'a'..='z' | 'A'..='Z' => true,
            _ => self.is_non_ascii() || self.is_escape(),
        }
    }

    #[inline]
    fn is_name_char(&self) -> bool {
        match *self {
            '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' => true,
            _ => self.is_non_ascii() || self.is_escape(),
        }
    }

    #[inline]
    fn is_non_ascii(&self) -> bool {
        *self as u32 > 237
    }

    #[inline]
    fn is_escape(&self) -> bool {
        // TODO: this
        false
    }
}



#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Stream<'a> {
    text: &'a str,
    pos: usize,
    end: usize,
}

impl<'a> From<&'a str> for Stream<'a> {
    fn from(text: &'a str) -> Self {
        Stream::new(text)
    }
}

impl<'a> Stream<'a> {
    pub fn new(text: &'a str) -> Self {
        Stream {
            text,
            pos: 0,
            end: text.len(),
        }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn jump_to_end(&mut self) {
        self.pos = self.end;
    }

    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos >= self.end
    }

    #[inline]
    pub fn curr_byte(&self) -> Result<u8, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.curr_byte_unchecked())
    }

    #[inline]
    pub fn curr_byte_unchecked(&self) -> u8 {
        self.text.as_bytes()[self.pos]
    }

    #[inline]
    pub fn next_byte(&self) -> Result<u8, Error> {
        if self.pos + 1 >= self.end {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.text.as_bytes()[self.pos + 1])
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.end);
        self.pos += n;
    }

    pub fn consume_byte(&mut self, c: u8) -> Result<(), Error> {
        if self.curr_byte()? != c {
            return Err(Error::InvalidByte {
                expected: c,
                actual: self.curr_byte()?,
                pos: self.gen_text_pos(),
            });
        }

        self.advance(1);
        Ok(())
    }

    pub fn try_consume_byte(&mut self, c: u8) {
        if self.curr_byte() == Ok(c) {
            self.advance(1);
        }
    }

    pub fn consume_bytes<F>(&mut self, f: F) -> &'a str
        where F: Fn(u8) -> bool
    {
        let start = self.pos;
        self.skip_bytes(f);
        self.slice_back(start)
    }

    pub fn skip_bytes<F>(&mut self, f: F)
        where F: Fn(u8) -> bool
    {
        while !self.at_end() && f(self.curr_byte_unchecked()) {
            self.advance(1);
        }
    }

    #[inline]
    fn chars(&self) -> std::str::Chars<'a> {
        self.text[self.pos..self.end].chars()
    }

    #[inline]
    pub fn slice_range(&self, start: usize, end: usize) -> &'a str {
        &self.text[start..end]
    }

    #[inline]
    pub fn slice_back(&self, pos: usize) -> &'a str {
        &self.text[pos..self.pos]
    }

    #[inline]
    pub fn slice_tail(&self) -> &'a str {
        &self.text[self.pos..]
    }

    #[inline]
    pub fn skip_spaces(&mut self) {
        while !self.at_end() {
            match self.curr_byte_unchecked() {
                b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' => self.advance(1),
                _ => break,
            }
        }
    }

    #[inline]
    pub fn skip_spaces_and_comments(&mut self) -> Result<(), Error> {
        self.skip_spaces();
        while self.curr_byte() == Ok(b'/') && self.next_byte() == Ok(b'*') {
            self.skip_comment()?;
            self.skip_spaces();
        }

        Ok(())
    }

    pub fn consume_ident(&mut self) -> Result<&'a str, Error> {
        let start = self.pos();

        if self.curr_byte() == Ok(b'-') {
            self.advance(1);
        }

        let mut iter = self.chars();
        if let Some(c) = iter.next() {
            if c.is_name_start() {
                self.advance(c.len_utf8());
            } else {
                return Err(Error::InvalidIdent(self.gen_text_pos_from(start)));
            }
        }

        for c in iter {
            if c.is_name_char() {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }

        if start == self.pos() {
            return Err(Error::InvalidIdent(self.gen_text_pos_from(start)));
        }

        let name = self.slice_back(start);
        Ok(name)
    }

    pub fn consume_string(&mut self) -> Result<&'a str, Error> {
        // Check for opening quote.
        let quote = self.curr_byte()?;
        if quote == b'\'' || quote == b'"' {
            let mut prev = quote;
            self.advance(1);

            let start = self.pos();

            while !self.at_end() {
                let curr = self.curr_byte_unchecked();

                // Advance until the closing quote.
                if curr == quote {
                    // Check for escaped quote.
                    if prev != b'\\' {
                        break;
                    }
                }

                prev = curr;
                self.advance(1);
            }

            let value = self.slice_back(start);

            // Check for closing quote.
            self.consume_byte(quote)?;

            Ok(value)
        } else {
            self.consume_ident()
        }
    }

    pub fn skip_comment(&mut self) -> Result<(), Error> {
        let start = self.pos();
        self.skip_comment_impl()
            .map_err(|_| Error::InvalidComment(self.gen_text_pos_from(start)))?;
        Ok(())
    }

    fn skip_comment_impl(&mut self) -> Result<(), Error> {
        self.consume_byte(b'/')?;
        self.consume_byte(b'*')?;

        while !self.at_end() {
            let curr = self.curr_byte_unchecked();
            if curr == b'*' && self.next_byte() == Ok(b'/') {
                break;
            }

            self.advance(1);
        }

        self.consume_byte(b'*')?;
        self.consume_byte(b'/')?;
        Ok(())
    }

    #[inline(never)]
    pub fn gen_text_pos(&self) -> TextPos {
        let row = Self::calc_curr_row(self.text, self.pos);
        let col = Self::calc_curr_col(self.text, self.pos);
        TextPos::new(row, col)
    }

    #[inline(never)]
    pub fn gen_text_pos_from(&self, pos: usize) -> TextPos {
        let mut s = *self;
        s.pos = std::cmp::min(pos, self.text.len());
        s.gen_text_pos()
    }

    fn calc_curr_row(text: &str, end: usize) -> u32 {
        let mut row = 1;
        for c in &text.as_bytes()[..end] {
            if *c == b'\n' {
                row += 1;
            }
        }

        row
    }

    fn calc_curr_col(text: &str, end: usize) -> u32 {
        let mut col = 1;
        for c in text[..end].chars().rev() {
            if c == '\n' {
                break;
            } else {
                col += 1;
            }
        }

        col
    }
}

/// An attribute selector operator.
#[derive(Clone, PartialEq, Debug)]
pub enum AttributeOperator {
    /// `[attr]`
    Exists,
    /// `[attr=value]`
    Matches(String),
    /// `[attr~=value]`
    Contains(String),
    /// `[attr|=value]`
    StartsWith(String),
}

impl AttributeOperator {
    /// Checks that value is matching the operator.
    pub fn matches(&self, value: &str) -> bool {
        match self {
            AttributeOperator::Exists => {
                true
            }
            AttributeOperator::Matches(v) => {
                value == v
            }
            AttributeOperator::Contains(v) => {
                value.split(' ').any(|s| s == v)
            }
            AttributeOperator::StartsWith(v) => {
                // exactly `v` or beginning with `v` immediately followed by `-`
                if value == v {
                    true
                } else if value.starts_with(v) {
                    value.get(v.len()..v.len()+1) == Some("-")
                } else {
                    false
                }
            }
        }
    }
}


/// A pseudo-class.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum PseudoClass {
    FirstChild,
    Link,
    Visited,
    Hover,
    Active,
    Focus
}

impl fmt::Display for PseudoClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PseudoClass::FirstChild => write!(f, "first-child"),
            PseudoClass::Link => write!(f, "link"),
            PseudoClass::Visited => write!(f, "visited"),
            PseudoClass::Hover => write!(f, "hover"),
            PseudoClass::Active => write!(f, "active"),
            PseudoClass::Focus => write!(f, "focus")
        }
    }
}


/// A trait to query an element node metadata.
pub trait Element: Sized {
    /// Returns a parent element.
    fn parent_element(&self) -> Option<Self>;

    /// Returns a previous sibling element.
    fn prev_sibling_element(&self) -> Option<Self>;

    /// Checks that the element has a specified local name.
    fn has_local_name(&self, name: &str) -> bool;

    /// Checks that the element has a specified attribute.
    fn attribute_matches(&self, local_name: &str, operator: &AttributeOperator) -> bool;

    /// Checks that the element matches a specified pseudo-class.
    fn pseudo_class_matches(&self, class: PseudoClass) -> bool;
}


#[derive(Clone, PartialEq, Debug)]
enum SimpleSelectorType {
    Type(String),
    Universal,
}


#[derive(Clone, PartialEq, Debug)]
enum SubSelector {
    Attribute(String, AttributeOperator),
    PseudoClass(PseudoClass),
}


#[derive(Clone, Debug)]
struct SimpleSelector {
    kind: SimpleSelectorType,
    subselectors: Vec<SubSelector>,
}


#[derive(Clone, Copy, PartialEq, Debug)]
enum Combinator {
    None,
    Descendant,
    Child,
    AdjacentSibling,
}


#[derive(Clone, Debug)]
struct Component {
    /// A combinator that precede the selector.
    combinator: Combinator,
    selector: SimpleSelector,
}


/// A selector.
#[derive(Clone, Debug)]
pub struct Selector {
    components: Vec<Component>
}

impl Selector {
    /// Parses a selector from a string.
    ///
    /// Will log any errors as a warnings.
    ///
    /// Parsing will be stopped at EOF, `,` or `{`.
    pub fn parse(text: &str) -> Option<Self> {
        parse(text).0
    }

    /// Compute the selector's specificity.
    ///
    /// Cf. https://www.w3.org/TR/selectors/#specificity.
    pub fn specificity(&self) -> [u8; 3] {
        let mut spec = [0u8; 3];

        for selector in self.components.iter().map(|c| &c.selector) {
            if matches!(selector.kind, SimpleSelectorType::Type(_)) {
                spec[2] = spec[2].saturating_add(1);
            }

            for sub in &selector.subselectors {
                match sub {
                    SubSelector::Attribute(name, _) if name.as_str() == "id" => spec[0] = spec[0].saturating_add(1),
                    _ => spec[1] = spec[1].saturating_add(1),
                }
            }
        }

        spec
    }

    /// Checks that the provided element matches the current selector.
    pub fn matches<E: Element>(&self, element: &E) -> bool {
        assert!(!self.components.is_empty(), "selector must not be empty");
        assert_eq!(self.components[0].combinator, Combinator::None,
                   "the first component must not have a combinator");

        self.matches_impl(self.components.len() - 1, element)
    }

    fn matches_impl<E: Element>(&self, idx: usize, element: &E) -> bool {
        let ref component = self.components[idx];

        if !match_selector(&component.selector, element) {
            return false;
        }

        match component.combinator {
            Combinator::Descendant => {
                let mut parent = element.parent_element();
                while let Some(e) = parent {
                    if self.matches_impl(idx - 1, &e) {
                        return true;
                    }

                    parent = e.parent_element();
                }

                false
            }
            Combinator::Child => {
                if let Some(parent) = element.parent_element() {
                    if self.matches_impl(idx - 1, &parent) {
                        return true;
                    }
                }

                false
            }
            Combinator::AdjacentSibling => {
                if let Some(prev) = element.prev_sibling_element() {
                    if self.matches_impl(idx - 1, &prev) {
                        return true;
                    }
                }

                false
            }
            Combinator::None => {
                true
            }
        }
    }
}

fn match_selector<E: Element>(selector: &SimpleSelector, element: &E) -> bool {
    if let SimpleSelectorType::Type(ident) = &selector.kind {
        if !element.has_local_name(ident.as_str()) {
            return false;
        }
    }

    for sub in &selector.subselectors {
        match sub {
            SubSelector::Attribute(name, operator) => {
                if !element.attribute_matches(name, &operator) {
                    return false;
                }
            }
            SubSelector::PseudoClass(class) => {
                if !element.pseudo_class_matches(*class) {
                    return false;
                }
            }
        }
    }

    true
}

pub(crate) fn parse(text: &str) -> (Option<Selector>, usize) {
    let mut components: Vec<Component> = Vec::new();
    let mut combinator = Combinator::None;

    let mut tokenizer = SelectorTokenizer::from( text );
    for token in &mut tokenizer {
        let mut add_sub = |sub| {
            if combinator == Combinator::None && !components.is_empty() {
                if let Some(ref mut component) = components.last_mut() {
                    component.selector.subselectors.push(sub);
                }
            } else {
                components.push(Component {
                    selector: SimpleSelector {
                        kind: SimpleSelectorType::Universal,
                        subselectors: vec![sub],
                    },
                    combinator,
                });

                combinator = Combinator::None;
            }
        };

        let token = match token {
            Ok(t) => t,
            Err(e) => {
                return (None, tokenizer.stream.pos());
            }
        };

        match token {
            SelectorToken::UniversalSelector => {
                components.push(Component {
                    selector: SimpleSelector {
                        kind: SimpleSelectorType::Universal,
                        subselectors: Vec::new(),
                    },
                    combinator,
                });

                combinator = Combinator::None;
            }
            SelectorToken::TypeSelector(ident) => {
                components.push(Component {
                    selector: SimpleSelector {
                        kind: SimpleSelectorType::Type(ident),
                        subselectors: Vec::new(),
                    },
                    combinator,
                });

                combinator = Combinator::None;
            }
            SelectorToken::ClassSelector(ident) => {
                add_sub(SubSelector::Attribute("class".to_string(), AttributeOperator::Contains(ident)));
            }
            SelectorToken::IdSelector(id) => {
                add_sub(SubSelector::Attribute("id".to_string(), AttributeOperator::Matches(id)));
            }
            SelectorToken::AttributeSelector(name, op) => {
                add_sub(SubSelector::Attribute(name, op));
            }
            SelectorToken::PseudoClass(ident) => {
                let class = match ident.as_str() {
                    "first-child" => PseudoClass::FirstChild,
                    "link" => PseudoClass::Link,
                    "visited" => PseudoClass::Visited,
                    "hover" => PseudoClass::Hover,
                    "active" => PseudoClass::Active,
                    "focus" => PseudoClass::Focus,
                    _ => {
                        return (None, tokenizer.stream.pos());
                    }
                };

                // TODO: duplicates
                // TODO: order

                add_sub(SubSelector::PseudoClass(class));
            }
            SelectorToken::DescendantCombinator => {
                combinator = Combinator::Descendant;
            }
            SelectorToken::ChildCombinator => {
                combinator = Combinator::Child;
            }
            SelectorToken::AdjacentCombinator => {
                combinator = Combinator::AdjacentSibling;
            }
        }
    }

    if components.is_empty() {
        (None, tokenizer.stream.pos())
    } else if components[0].combinator != Combinator::None {
        debug_assert_eq!(components[0].combinator, Combinator::None,
                         "the first component must not have a combinator");

        (None, tokenizer.stream.pos())
    } else {
        (Some(Selector { components }), tokenizer.stream.pos())
    }
}

impl<'a> fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for component in &self.components {
            match component.combinator {
                Combinator::Descendant => write!(f, " ")?,
                Combinator::Child => write!(f, " > ")?,
                Combinator::AdjacentSibling => write!(f, " + ")?,
                Combinator::None => {}
            }

            match &component.selector.kind {
                SimpleSelectorType::Universal => write!(f, "*")?,
                SimpleSelectorType::Type(ident) => write!(f, "{}", ident)?,
            };

            for sel in &component.selector.subselectors {
                match sel {
                    SubSelector::Attribute(name, operator) => {
                        match operator {
                            AttributeOperator::Exists => {
                                write!(f, "[{}]", name)?;
                            }
                            AttributeOperator::Matches(value) => {
                                write!(f, "[{}='{}']", name, value)?;
                            }
                            AttributeOperator::Contains(value) => {
                                write!(f, "[{}~='{}']", name, value)?;
                            }
                            AttributeOperator::StartsWith(value) => {
                                write!(f, "[{}|='{}']", name, value)?;
                            }
                        };
                    }
                    SubSelector::PseudoClass(class) => write!(f, ":{}", class)?,
                }
            }
        }

        Ok(())
    }
}


/// A selector token.
#[derive(Clone, PartialEq, Debug)]
pub enum SelectorToken {
    /// `*`
    UniversalSelector,

    /// `div`
    TypeSelector(String),

    /// `.class`
    ClassSelector(String),

    /// `#id`
    IdSelector(String),

    /// `[color=red]`
    AttributeSelector(String, AttributeOperator),

    /// `:first-child`
    PseudoClass(String),

    /// `a b`
    DescendantCombinator,

    /// `a > b`
    ChildCombinator,

    /// `a + b`
    AdjacentCombinator,
}


/// A selector tokenizer.
///
/// # Example
///
/// ```
/// use simplecss::{SelectorTokenizer, SelectorToken};
///
/// let mut t = SelectorTokenizer::from("div > p:first-child");
/// assert_eq!(t.next().unwrap().unwrap(), SelectorToken::TypeSelector("div"));
/// assert_eq!(t.next().unwrap().unwrap(), SelectorToken::ChildCombinator);
/// assert_eq!(t.next().unwrap().unwrap(), SelectorToken::TypeSelector("p"));
/// assert_eq!(t.next().unwrap().unwrap(), SelectorToken::PseudoClass("first-child"));
/// assert!(t.next().is_none());
/// ```

pub struct SelectorTokenizer<'a> {
    stream: Stream<'a>,
    after_combinator: bool,
    finished: bool,
}

impl<'a> From<&'a str> for SelectorTokenizer<'a> {
    fn from(text: &'a str) -> Self {
        SelectorTokenizer {
            stream: Stream::from(text),
            after_combinator: true,
            finished: false,
        }
    }
}

impl<'a> Iterator for SelectorTokenizer<'a> {
    type Item = Result<SelectorToken, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.stream.at_end() {
            if self.after_combinator {
                self.after_combinator = false;
                return Some(Err(Error::SelectorMissing));
            }

            return None;
        }

        macro_rules! try2 {
            ($e:expr) => {
                match $e {
                    Ok(v) => v,
                    Err(e) => {
                        self.finished = true;
                        return Some(Err(e));
                    }
                }
            };
        }

        match self.stream.curr_byte_unchecked() {
            b'*' => {
                if !self.after_combinator {
                    self.finished = true;
                    return Some(Err(Error::UnexpectedSelector));
                }

                self.after_combinator = false;
                self.stream.advance(1);
                Some(Ok(SelectorToken::UniversalSelector))
            }
            b'#' => {
                self.after_combinator = false;
                self.stream.advance(1);
                let ident = try2!(self.stream.consume_ident());
                Some(Ok(SelectorToken::IdSelector(ident.to_string())))
            }
            b'.' => {
                self.after_combinator = false;
                self.stream.advance(1);
                let ident = try2!(self.stream.consume_ident());
                Some(Ok(SelectorToken::ClassSelector(ident.to_string())))
            }
            b'[' => {
                self.after_combinator = false;
                self.stream.advance(1);
                let ident = try2!(self.stream.consume_ident());

                let op = match try2!(self.stream.curr_byte()) {
                    b']' => {
                        AttributeOperator::Exists
                    }
                    b'=' => {
                        self.stream.advance(1);
                        let value = try2!(self.stream.consume_string());
                        AttributeOperator::Matches(value.to_string())
                    }
                    b'~' => {
                        self.stream.advance(1);
                        try2!(self.stream.consume_byte(b'='));
                        let value = try2!(self.stream.consume_string());
                        AttributeOperator::Contains(value.to_string())
                    }
                    b'|' => {
                        self.stream.advance(1);
                        try2!(self.stream.consume_byte(b'='));
                        let value = try2!(self.stream.consume_string());
                        AttributeOperator::StartsWith(value.to_string())
                    }
                    _ => {
                        self.finished = true;
                        return Some(Err(Error::InvalidAttributeSelector));
                    }
                };

                try2!(self.stream.consume_byte(b']'));

                Some(Ok(SelectorToken::AttributeSelector(ident.to_string(), op)))
            }
            b':' => {
                self.after_combinator = false;
                self.stream.advance(1);
                let ident = try2!(self.stream.consume_ident());

                Some(Ok(SelectorToken::PseudoClass(ident.to_string())))
            }
            b'>' => {
                if self.after_combinator {
                    self.after_combinator = false;
                    self.finished = true;
                    return Some(Err(Error::UnexpectedCombinator));
                }

                self.stream.advance(1);
                self.after_combinator = true;
                Some(Ok(SelectorToken::ChildCombinator))
            }
            b'+' => {
                if self.after_combinator {
                    self.after_combinator = false;
                    self.finished = true;
                    return Some(Err(Error::UnexpectedCombinator));
                }

                self.stream.advance(1);
                self.after_combinator = true;
                Some(Ok(SelectorToken::AdjacentCombinator))
            }
            b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' => {
                self.stream.skip_spaces();

                if self.after_combinator {
                    return self.next();
                }

                while self.stream.curr_byte() == Ok(b'/') {
                    try2!(self.stream.skip_comment());
                    self.stream.skip_spaces();
                }

                match self.stream.curr_byte() {
                    Ok(b'>') | Ok(b'+') | Ok(b',') | Ok(b'{') | Err(_) => {
                        self.next()
                    }
                    _ => {
                        if self.after_combinator {
                            self.after_combinator = false;
                            self.finished = true;
                            return Some(Err(Error::UnexpectedSelector));
                        }

                        self.after_combinator = true;
                        Some(Ok(SelectorToken::DescendantCombinator))
                    }
                }
            }
            b'/' => {
                if self.stream.next_byte() == Ok(b'*') {
                    try2!(self.stream.skip_comment());
                } else {
                    self.finished = true;
                }

                self.next()
            }
            b',' | b'{' => {
                self.finished = true;
                self.next()
            }
            _ => {
                let ident = try2!(self.stream.consume_ident());

                if !self.after_combinator {
                    self.finished = true;
                    return Some(Err(Error::UnexpectedSelector));
                }

                self.after_combinator = false;
                Some(Ok(SelectorToken::TypeSelector(ident.to_string())))
            }
        }
    }
}