// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::{Element, Renderer};
use iced::widget::scrollable::{Direction, Scrollable, Scrollbar};

pub const SCROLLER_WIDTH: u16 = 8;
pub const SCROLLBAR_WIDTH: u16 = 8;

pub fn vertical<'a, Message>(
    element: impl Into<Element<'a, Message>>,
) -> Scrollable<'a, Message, crate::Theme, Renderer> {
    iced::widget::scrollable(element).direction(Direction::Vertical(
        Scrollbar::new()
            .width(SCROLLBAR_WIDTH)
            .scroller_width(SCROLLER_WIDTH)
            .padding(8),
    ))
}

pub fn horizontal<'a, Message>(
    element: impl Into<Element<'a, Message>>,
) -> Scrollable<'a, Message, crate::Theme, Renderer> {
    iced::widget::scrollable(element).direction(Direction::Horizontal(
        Scrollbar::new()
            .width(SCROLLBAR_WIDTH)
            .scroller_width(SCROLLER_WIDTH),
    ))
}
