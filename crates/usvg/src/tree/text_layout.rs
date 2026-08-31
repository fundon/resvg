// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{FontStretch, FontStyle};

impl From<fontdb::Stretch> for FontStretch {
    fn from(stretch: fontdb::Stretch) -> Self {
        match stretch {
            fontdb::Stretch::UltraCondensed => FontStretch::UltraCondensed,
            fontdb::Stretch::ExtraCondensed => FontStretch::ExtraCondensed,
            fontdb::Stretch::Condensed => FontStretch::Condensed,
            fontdb::Stretch::SemiCondensed => FontStretch::SemiCondensed,
            fontdb::Stretch::Normal => FontStretch::Normal,
            fontdb::Stretch::SemiExpanded => FontStretch::SemiExpanded,
            fontdb::Stretch::Expanded => FontStretch::Expanded,
            fontdb::Stretch::ExtraExpanded => FontStretch::ExtraExpanded,
            fontdb::Stretch::UltraExpanded => FontStretch::UltraExpanded,
        }
    }
}

impl From<FontStretch> for fontdb::Stretch {
    fn from(stretch: FontStretch) -> Self {
        match stretch {
            FontStretch::UltraCondensed => fontdb::Stretch::UltraCondensed,
            FontStretch::ExtraCondensed => fontdb::Stretch::ExtraCondensed,
            FontStretch::Condensed => fontdb::Stretch::Condensed,
            FontStretch::SemiCondensed => fontdb::Stretch::SemiCondensed,
            FontStretch::Normal => fontdb::Stretch::Normal,
            FontStretch::SemiExpanded => fontdb::Stretch::SemiExpanded,
            FontStretch::Expanded => fontdb::Stretch::Expanded,
            FontStretch::ExtraExpanded => fontdb::Stretch::ExtraExpanded,
            FontStretch::UltraExpanded => fontdb::Stretch::UltraExpanded,
        }
    }
}

impl From<fontdb::Style> for FontStyle {
    fn from(style: fontdb::Style) -> Self {
        match style {
            fontdb::Style::Normal => FontStyle::Normal,
            fontdb::Style::Italic => FontStyle::Italic,
            fontdb::Style::Oblique => FontStyle::Oblique,
        }
    }
}

impl From<FontStyle> for fontdb::Style {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => fontdb::Style::Normal,
            FontStyle::Italic => fontdb::Style::Italic,
            FontStyle::Oblique => fontdb::Style::Oblique,
        }
    }
}
