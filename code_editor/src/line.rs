use {
    crate::{
        token::{TokenInfo, TokenKind},
        Bias, BiasedLinePos, GridPos, Token,
    },
    std::slice,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line<'a> {
    text: &'a str,
    token_infos: &'a [TokenInfo],
    text_inlays: &'a [(usize, String)],
    widget_inlays: &'a [((usize, Bias), Widget)],
    wrap_bytes: &'a [usize],
    start_column_after_wrap: usize,
    fold_column: usize,
    scale: f64,
}

impl<'a> Line<'a> {
    pub fn new(
        text: &'a str,
        token_infos: &'a [TokenInfo],
        text_inlays: &'a [(usize, String)],
        widget_inlays: &'a [((usize, Bias), Widget)],
        wrap_bytes: &'a [usize],
        start_column_after_wrap: usize,
        fold_column: usize,
        scale: f64,
    ) -> Self {
        Self {
            text,
            token_infos,
            text_inlays,
            widget_inlays,
            wrap_bytes,
            start_column_after_wrap,
            fold_column,
            scale,
        }
    }

    pub fn compute_column_count(&self, tab_column_count: usize) -> usize {
        use crate::str::StrExt;

        let mut max_summed_column_count = 0;
        let mut summed_column_count = 0;
        for wrapped_element in self.wrapped_elements() {
            match wrapped_element {
                WrappedElement::Token(_, token) => {
                    summed_column_count += token.text.column_count(tab_column_count);
                }
                WrappedElement::Widget(_, widget) => {
                    summed_column_count += widget.column_count;
                }
                WrappedElement::Wrap => {
                    max_summed_column_count = max_summed_column_count.max(summed_column_count);
                    summed_column_count = self.start_column_after_wrap();
                }
            }
        }
        max_summed_column_count.max(summed_column_count)
    }

    pub fn row_count(&self) -> usize {
        self.wrap_bytes.len() + 1
    }

    pub fn compute_width(&self, tab_column_count: usize) -> f64 {
        self.column_to_x(self.compute_column_count(tab_column_count))
    }

    pub fn height(&self) -> f64 {
        self.scale * self.row_count() as f64
    }

    pub fn pos_to_grid_pos(&self, pos: BiasedLinePos, tab_column_count: usize) -> GridPos {
        use crate::str::StrExt;

        let mut current_pos = 0;
        let mut grid_pos = GridPos::default();
        if pos
            == (BiasedLinePos {
                pos: current_pos,
                bias: Bias::Before,
            })
        {
            return grid_pos;
        }
        for wrapped_element in self.wrapped_elements() {
            match wrapped_element {
                WrappedElement::Token(false, token) => {
                    for grapheme in token.text.graphemes() {
                        if pos
                            == (BiasedLinePos {
                                pos: current_pos,
                                bias: Bias::After,
                            })
                        {
                            return grid_pos;
                        }
                        current_pos += grapheme.len();
                        grid_pos.col += grapheme.column_count(tab_column_count);
                        if pos
                            == (BiasedLinePos {
                                pos: current_pos,
                                bias: Bias::Before,
                            })
                        {
                            return grid_pos;
                        }
                    }
                }
                WrappedElement::Token(true, token) => {
                    grid_pos.col += token.text.column_count(tab_column_count);
                }
                WrappedElement::Widget(_, widget) => {
                    grid_pos.col += widget.column_count;
                }
                WrappedElement::Wrap => {
                    grid_pos.row += 1;
                    grid_pos.col = self.start_column_after_wrap();
                }
            }
        }
        if pos
            == (BiasedLinePos {
                pos: current_pos,
                bias: Bias::After,
            })
        {
            return grid_pos;
        }
        panic!()
    }

    pub fn grid_pos_to_pos(&self, grid_pos: GridPos, tab_column_count: usize) -> BiasedLinePos {
        use crate::str::StrExt;

        let mut row = 0;
        let mut col = 0;
        let mut pos = 0;
        for wrapped_element in self.wrapped_elements() {
            match wrapped_element {
                WrappedElement::Token(false, token) => {
                    for grapheme in token.text.graphemes() {
                        let next_column = col + grapheme.column_count(tab_column_count);
                        if grid_pos.row == row && (col..next_column).contains(&grid_pos.col) {
                            return BiasedLinePos {
                                pos,
                                bias: Bias::After,
                            };
                        }
                        pos = pos + grapheme.len();
                        col = next_column;
                    }
                }
                WrappedElement::Token(true, token) => {
                    let next_column = col + token.text.column_count(tab_column_count);
                    if grid_pos.row == row && (col..next_column).contains(&grid_pos.col) {
                        return BiasedLinePos {
                            pos,
                            bias: Bias::Before,
                        };
                    }
                    col = next_column;
                }
                WrappedElement::Widget(_, widget) => {
                    col += widget.column_count;
                }
                WrappedElement::Wrap => {
                    if grid_pos.row == row {
                        return BiasedLinePos {
                            pos,
                            bias: Bias::Before,
                        };
                    }
                    row += 1;
                    col = self.start_column_after_wrap();
                }
            }
        }
        if grid_pos.row == row {
            return BiasedLinePos {
                pos,
                bias: Bias::After,
            };
        }
        panic!()
    }

    pub fn column_to_x(&self, column: usize) -> f64 {
        let column_count_before_fold_column = column.min(self.fold_column);
        let column_count_after_fold_column = column - column_count_before_fold_column;
        column_count_before_fold_column as f64 + self.scale * column_count_after_fold_column as f64
    }

    pub fn text(&self) -> &'a str {
        self.text
    }

    pub fn tokens(&self) -> Tokens<'a> {
        Tokens {
            text: self.text,
            token_infos: self.token_infos.iter(),
        }
    }

    pub fn elements(&self) -> Elements<'a> {
        let mut tokens = self.tokens();
        Elements {
            token: tokens.next(),
            tokens,
            text_inlays: self.text_inlays,
            widget_inlays: self.widget_inlays,
            byte: 0,
        }
    }

    pub fn wrapped_elements(&self) -> WrappedElements<'a> {
        let mut elements = self.elements();
        WrappedElements {
            element: elements.next(),
            elements,
            wrap_bytes: self.wrap_bytes,
            byte: 0,
        }
    }

    pub fn start_column_after_wrap(&self) -> usize {
        self.start_column_after_wrap
    }

    pub fn fold_column(&self) -> usize {
        self.fold_column
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}

#[derive(Clone, Debug)]
pub struct Tokens<'a> {
    text: &'a str,
    token_infos: slice::Iter<'a, TokenInfo>,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.token_infos.next() {
            Some(token_info) => {
                let (text_0, text_1) = self.text.split_at(token_info.byte_count);
                self.text = text_1;
                Token::new(text_0, token_info.kind)
            }
            None => {
                if self.text.is_empty() {
                    return None;
                }
                let text = self.text;
                self.text = "";
                Token::new(text, TokenKind::Unknown)
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct Elements<'a> {
    token: Option<Token<'a>>,
    tokens: Tokens<'a>,
    text_inlays: &'a [(usize, String)],
    widget_inlays: &'a [((usize, Bias), Widget)],
    byte: usize,
}

impl<'a> Iterator for Elements<'a> {
    type Item = Element<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self
            .widget_inlays
            .first()
            .map_or(false, |((byte, bias), _)| {
                *byte == self.byte && *bias == Bias::Before
            })
        {
            let ((_, widget), widget_inlays) = self.widget_inlays.split_first().unwrap();
            self.widget_inlays = widget_inlays;
            return Some(Element::Widget(Bias::Before, *widget));
        }
        if self
            .text_inlays
            .first()
            .map_or(false, |(byte, _)| *byte == self.byte)
        {
            let ((_, text), text_inlays) = self.text_inlays.split_first().unwrap();
            self.text_inlays = text_inlays;
            return Some(Element::Token(true, Token::new(text, TokenKind::Unknown)));
        }
        if self
            .widget_inlays
            .first()
            .map_or(false, |((byte, bias), _)| {
                *byte == self.byte && *bias == Bias::After
            })
        {
            let ((_, widget), widget_inlays) = self.widget_inlays.split_first().unwrap();
            self.widget_inlays = widget_inlays;
            return Some(Element::Widget(Bias::After, *widget));
        }
        let token = self.token.take()?;
        let mut byte_count = token.text.len();
        if let Some((byte, _)) = self.text_inlays.first() {
            byte_count = byte_count.min(*byte - self.byte);
        }
        if let Some(((byte, _), _)) = self.widget_inlays.first() {
            byte_count = byte_count.min(byte - self.byte);
        }
        let token = if byte_count < token.text.len() {
            let (text_0, text_1) = token.text.split_at(byte_count);
            self.token = Some(Token::new(text_1, token.kind));
            Token::new(text_0, token.kind)
        } else {
            self.token = self.tokens.next();
            token
        };
        self.byte += token.text.len();
        Some(Element::Token(false, token))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Element<'a> {
    Token(bool, Token<'a>),
    Widget(Bias, Widget),
}

#[derive(Clone, Debug)]
pub struct WrappedElements<'a> {
    element: Option<Element<'a>>,
    elements: Elements<'a>,
    wrap_bytes: &'a [usize],
    byte: usize,
}

impl<'a> Iterator for WrappedElements<'a> {
    type Item = WrappedElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Element::Widget(Bias::Before, ..)) = self.element {
            let Element::Widget(_, widget) = self.element.take().unwrap() else {
                panic!()
            };
            self.element = self.elements.next();
            return Some(WrappedElement::Widget(Bias::Before, widget));
        }
        if self
            .wrap_bytes
            .first()
            .map_or(false, |byte| *byte == self.byte)
        {
            self.wrap_bytes = &self.wrap_bytes[1..];
            return Some(WrappedElement::Wrap);
        }
        Some(match self.element.take()? {
            Element::Token(is_inlay, token) => {
                let mut byte_count = token.text.len();
                if let Some(byte) = self.wrap_bytes.first() {
                    byte_count = byte_count.min(*byte - self.byte);
                }
                let token = if byte_count < token.text.len() {
                    let (text_0, text_1) = token.text.split_at(byte_count);
                    self.element = Some(Element::Token(is_inlay, Token::new(text_1, token.kind)));
                    Token::new(text_0, token.kind)
                } else {
                    self.element = self.elements.next();
                    token
                };
                self.byte += token.text.len();
                WrappedElement::Token(is_inlay, token)
            }
            Element::Widget(Bias::After, widget) => {
                self.element = self.elements.next();
                WrappedElement::Widget(Bias::After, widget)
            }
            Element::Widget(Bias::Before, _) => panic!(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WrappedElement<'a> {
    Token(bool, Token<'a>),
    Widget(Bias, Widget),
    Wrap,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Widget {
    pub id: usize,
    pub column_count: usize,
}

impl Widget {
    pub fn new(id: usize, column_count: usize) -> Self {
        Self { id, column_count }
    }
}