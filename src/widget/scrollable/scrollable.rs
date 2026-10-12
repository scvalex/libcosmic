// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::{Element, Renderer};
use iced::widget::scrollable::{Direction, Scrollable, Scrollbar};

pub const SCROLLBAR_WIDTH: f32 = 8.0;
pub const SCROLLER_WIDTH: f32 = 8.0;

pub fn scrollable<'a, Message>(
    element: impl Into<Element<'a, Message>>,
) -> Scrollable<'a, Message, crate::Theme, Renderer> {
    vertical(element)
}

pub fn vertical<'a, Message>(
    element: impl Into<Element<'a, Message>>,
) -> Scrollable<'a, Message, crate::Theme, Renderer> {
    iced::widget::scrollable(element)
        .padding(8.0)
        .direction(Direction::Vertical(
            Scrollbar::new()
                .width(SCROLLBAR_WIDTH)
                .scroller_width(SCROLLER_WIDTH),
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
