// SPDX-License-Identifier: MIT
use super::symbols::block;
use tuirs::buffer::Buffer;
use tuirs::layout::Rect;
use tuirs::style::{Color, Modifier, Style};
use tuirs::symbols::line;
use tuirs::widgets::Widget;

#[derive(Clone, PartialEq, Eq)]
pub enum WaveformElement {
    Low,
    High,
    Value(String),
    Transition,
    RisingEdge,
    FallingEdge,
    Invalid,
    LowDensity,
    MediumDensity,
    HighDensity,
}

impl WaveformElement {
    pub fn to_symbols(&self) -> (&str, &str, &str) {
        match self {
            WaveformElement::Low => (" ", " ", line::HORIZONTAL),
            WaveformElement::High => (line::HORIZONTAL, " ", " "),
            WaveformElement::Value(_) => (line::HORIZONTAL, " ", line::HORIZONTAL),
            WaveformElement::RisingEdge => (line::TOP_LEFT, line::VERTICAL, line::BOTTOM_RIGHT),
            WaveformElement::FallingEdge => (line::TOP_RIGHT, line::VERTICAL, line::BOTTOM_LEFT),
            WaveformElement::Transition => {
                (line::HORIZONTAL_DOWN, line::VERTICAL, line::HORIZONTAL_UP)
            }
            WaveformElement::Invalid => (block::FULL_LOWER, block::FULL, block::FULL_UPPER),
            WaveformElement::LowDensity => (block::LIGHT_LOWER, block::LIGHT, block::LIGHT_UPPER),
            WaveformElement::MediumDensity => {
                (block::MEDIUM_LOWER, block::MEDIUM, block::MEDIUM_UPPER)
            },
            WaveformElement::HighDensity => (block::FULL_LOWER, block::FULL, block::FULL_UPPER),
        }
    }
}

pub struct Waveform<'a> {
    data: &'a [WaveformElement],
    name: String,
    selected: bool,
    cursor: usize,
    visual_cursor: Option<usize>
}

impl<'a> Waveform<'a> {
    pub fn new(
        name: String,
        data: &'a [WaveformElement],
        selected: bool,
        cursor: usize,
        visual_cursor: Option<usize>,
    ) -> Waveform<'a> {
        Waveform {
            data,
            name,
            selected,
            cursor,
            visual_cursor
        }
    }
}

impl<'a> Widget for Waveform<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        for (i, elmt) in self.data.iter().enumerate() {
            let fg = if i == self.cursor {
                if self.selected {
                    Color::White
                } else {
                    Color::Black
                }
            } else if *elmt == WaveformElement::Invalid {
                if self.selected {
                    Color::LightRed
                } else {
                    Color::Red
                }
            } else if self.selected {
                Color::LightGreen
            } else {
                Color::Green
            };
            let bg = if i == self.cursor {
                Color::Gray
            } else if let Some(visual_cursor) = self.visual_cursor {
                if (visual_cursor <= i && i <= self.cursor) ||
                    (self.cursor <= i && i <= visual_cursor) {
                    Color::Blue
                } else {
                    Color::Black
                }
            } else {
                Color::Black
            };
            let style = Style::default().fg(fg).bg(bg);

            let (top, middle, bottom) = elmt.to_symbols();
            buf.get_mut(area.left() + i as u16, area.top())
                .set_symbol(top)
                .set_style(style);
            buf.get_mut(area.left() + i as u16, area.top() + 1)
                .set_symbol(middle)
                .set_style(style);
            buf.get_mut(area.left() + i as u16, area.top() + 2)
                .set_symbol(bottom)
                .set_style(style);
        }

        let mut elmts = self.data.iter().enumerate();
        loop {
            let mut free_space = 0;
            let mut value = "";
            let mut elmt = elmts.next();
            let offset = if let Some((i, _)) = elmt {
                i
            } else {
                break
            };

            while let Some((_, WaveformElement::Value(v))) = elmt {
                free_space += 1;
                value = v;
                elmt = elmts.next();
            }


            for (i, c) in value.chars().enumerate() {
                if i >= free_space {
                    break
                }

                let r = &c.to_string();
                let symbol = if i >= free_space - 1 && i < value.len() { "…" } else { r };

                buf.get_mut(area.left() + (offset + i) as u16, area.top() + 1)
                    .set_symbol(&symbol);
            }
        }

        buf.set_stringn(
            area.left(),
            area.top(),
            &self.name,
            area.width as usize,
            Style::default()
                .bg(Color::DarkGray)
                .fg(if self.selected { Color::White } else { Color::Black })
                .modifier( if self.selected { Modifier::BOLD } else { Modifier::empty() }),
        );
    }
}
