use std::borrow::Cow;

use crate::widget::Icon;

/// Implementation of std::fmt::Display allows user to customize the header
/// Ideally, this is implemented on an enum.
pub trait ItemCategory:
    Default + std::fmt::Display + Clone + Copy + PartialEq + Eq + std::hash::Hash
{
    /// Get the width of the column
    fn width(&self) -> iced::Length;

    /// Get the horizontal alignment of the column.
    ///
    /// For example, numerical columns are traditionally alligned to the right.
    fn alignment(&self) -> iced::alignment::Horizontal {
        iced::alignment::Horizontal::Left
    }

    /// Get the font of the column.
    ///
    /// For example, set the first column to [`cosmic::font::bold`] to indicate
    /// a secondary header.
    fn font(&self) -> iced::Font {
        crate::font::default()
    }

    /// Get the horizontal spacing around the column.
    fn spacing(&self) -> iced::Pixels {
        iced::Pixels::ZERO
    }
}

pub trait ItemInterface<Category: ItemCategory> {
    fn get_icon(&self, category: Category) -> Option<Icon>;
    fn get_text(&self, category: Category) -> Cow<'static, str>;

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering;
}
