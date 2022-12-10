//! Containers style

use crate::enums::element_type::ElementType;
use crate::get_colors;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{BORDER_ROUNDED_RADIUS, BORDER_WIDTH};
use iced::Background;
use iced_style::container::Appearance;
use iced_style::Theme;

impl From<StyleTuple> for iced::theme::Container {
    fn from(tuple: StyleTuple) -> Self {
        iced_style::theme::Container::Custom(Box::new(tuple))
    }
}

impl iced_style::container::StyleSheet for StyleTuple {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            text_color: Option::Some(match self {
                StyleTuple(_, ElementType::Headers) => colors.text_headers,
                _ => colors.text_body,
            }),
            background: Option::Some(Background::Color(match self {
                StyleTuple(_, ElementType::Headers) => colors.secondary,
                _ => colors.primary,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::BorderedRound) => BORDER_ROUNDED_RADIUS,
                _ => 0.0,
            },
            border_width: match self {
                StyleTuple(_, ElementType::Standard | ElementType::Headers) => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: colors.round_borders,
        }
    }
}
