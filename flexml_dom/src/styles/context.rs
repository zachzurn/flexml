use bitflags::bitflags;
use logos::Source;
use paste::paste;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dimension {
    Px(i32),
    Percent(f32),
}

impl Default for Dimension {
    fn default() -> Dimension {
        Dimension::Px(0)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum FontFamily {
    #[default]
    SansSerif,
    Serif,
    Monospace,
    UserDefined(usize)
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Image {
    #[default]
    None,
    UserDefined(usize),
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Length {
    #[default]
    Auto,
    Content,
    Px(i32),
    Percent(f32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Block,
    Inline,
    InlineBlock,
    Flex,
    Table,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WhiteSpace {
    #[default]
    Normal,
    NoWrap,
    Pre,
    PreWrap,
    PreLine,
}


#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignContent {
    #[default]
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    Stretch,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignItems {
    #[default]
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignSelf {
    #[default]
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}


#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum JustifyContent {
    #[default]
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}


#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextDecoration {
    #[default]
    None,
    Underline,
    Overline,
    LineThrough,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextTransform {
    #[default]
    None,
    Capitalize,
    Uppercase,
    Lowercase,
}

#[repr(u8)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontWeight {
    #[default]
    Normal,
    Bold,
    Light,
    Bolder,
    Lighter,
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BgPosition {
    #[default]
    Center,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BgRepeat {
    Repeat,
    RepeatX,
    RepeatY,
    #[default]
    NoRepeat,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BgSize {
    Auto,
    Cover,
    #[default]
    Contain,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorderStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
    None,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8); // RGBA


bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq)]
    pub struct StyleBits: u64 {
        const DISPLAY              = 1 << 0;
        const WHITE_SPACE          = 1 << 1;
        const OPACITY              = 1 << 2;

        const MARGIN               = 1 << 3;
        const MARGIN_TOP          = 1 << 4;
        const MARGIN_BOTTOM       = 1 << 5;
        const MARGIN_LEFT         = 1 << 6;
        const MARGIN_RIGHT        = 1 << 7;

        const PADDING              = 1 << 8;
        const PADDING_TOP         = 1 << 9;
        const PADDING_BOTTOM      = 1 << 10;
        const PADDING_LEFT        = 1 << 11;
        const PADDING_RIGHT       = 1 << 12;

        const ALIGN_CONTENT        = 1 << 13;
        const ALIGN_ITEMS          = 1 << 14;
        const ALIGN_SELF           = 1 << 15;
        const GAP                  = 1 << 16;
        const COLUMN_GAP           = 1 << 17;
        const ROW_GAP              = 1 << 18;
        const FLEX_BASIS           = 1 << 19;
        const FLEX_DIRECTION       = 1 << 20;
        const FLEX_GROW            = 1 << 21;
        const FLEX_SHRINK          = 1 << 22;
        const JUSTIFY_CONTENT      = 1 << 23;
        const FLEX_WRAP            = 1 << 24;

        const WIDTH                = 1 << 25;
        const MAX_WIDTH            = 1 << 26;
        const MIN_WIDTH            = 1 << 27;
        const HEIGHT               = 1 << 28;
        const MAX_HEIGHT           = 1 << 29;
        const MIN_HEIGHT           = 1 << 30;

        const TEXT_ALIGN           = 1 << 31;
        const COLOR                = 1 << 32;
        const TEXT_DECORATION      = 1 << 33;
        const FONT_FAMILY          = 1 << 34;
        const FONT_SIZE            = 1 << 35;
        const FONT_STYLE           = 1 << 36;
        const TEXT_TRANSFORM       = 1 << 37;
        const LETTER_SPACING       = 1 << 38;
        const LINE_HEIGHT          = 1 << 39;
        const FONT_WEIGHT          = 1 << 40;
        const WORD_SPACING         = 1 << 41;

        const BG_COLOR             = 1 << 41;
        const BG_IMAGE             = 1 << 43;
        const BG_POSITION          = 1 << 44;
        const BG_REPEAT            = 1 << 45;
        const BG_SIZE              = 1 << 46;

        const BORDER_RADIUS        = 1 << 47;
        const BORDER_TOP_LEFT      = 1 << 48;
        const BORDER_TOP_RIGHT     = 1 << 49;
        const BORDER_BOTTOM_LEFT   = 1 << 50;
        const BORDER_BOTTOM_RIGHT  = 1 << 51;
        const BORDER_COLOR         = 1 << 52;
        const BORDER_STYLE         = 1 << 53;
        const BORDER_WIDTH         = 1 << 54;
    }
}

static INHERITABLE_STYLES: &[StyleBits] = &[
    StyleBits::COLOR,
    StyleBits::FONT_FAMILY,
    StyleBits::FONT_SIZE,
    StyleBits::FONT_STYLE,
    StyleBits::FONT_WEIGHT,
    StyleBits::LETTER_SPACING,
    StyleBits::LINE_HEIGHT,
    StyleBits::TEXT_ALIGN,
    StyleBits::TEXT_DECORATION,
    StyleBits::TEXT_TRANSFORM,
    StyleBits::WHITE_SPACE,
    StyleBits::WORD_SPACING,
];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StyleContext {
    pub bits: StyleBits,

    pub display: Display,
    pub white_space: WhiteSpace,
    pub opacity: f32,

    pub margin: Dimension,
    pub margin_top: Dimension,
    pub margin_bottom: Dimension,
    pub margin_left: Dimension,
    pub margin_right: Dimension,

    pub padding: Dimension,
    pub padding_top: Dimension,
    pub padding_bottom: Dimension,
    pub padding_left: Dimension,
    pub padding_right: Dimension,

    pub align_content: AlignContent,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub gap: Dimension,
    pub column_gap: Dimension,
    pub row_gap: Dimension,

    pub flex_basis: Length,
    pub flex_direction: FlexDirection,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub justify_content: JustifyContent,
    pub flex_wrap: FlexWrap,

    pub width: Dimension,
    pub max_width: Dimension,
    pub min_width: Dimension,
    pub height: Dimension,
    pub max_height: Dimension,
    pub min_height: Dimension,

    pub text_align: TextAlign,
    pub color: Color,
    pub text_decoration: TextDecoration,
    pub font_family: FontFamily,
    pub font_size: Dimension,
    pub font_style: FontStyle,
    pub text_transform: TextTransform,
    pub letter_spacing: Dimension,
    pub line_height: Dimension,
    pub font_weight: FontWeight,
    pub word_spacing: Dimension,

    pub bg_color: Color,
    pub bg_image: Image,
    pub bg_position: BgPosition,
    pub bg_repeat: BgRepeat,
    pub bg_size: BgSize,


    pub border_radius: Dimension,
    pub border_top_left_radius: Dimension,
    pub border_top_right_radius: Dimension,
    pub border_bottom_left_radius: Dimension,
    pub border_bottom_right_radius: Dimension,
    pub border_color: Color,
    pub border_style: BorderStyle,
    pub border_width: Dimension,
}

macro_rules! style_field {
    ($field:ident : $ty:ty, $bit:expr, $default:expr) => {
        paste! {
            pub fn [<set_ $field>](&mut self, value: $ty) {
                self.$field = value;
                self.bits.insert($bit);
            }
            pub fn [<has_ $field>](&self) -> bool {
                self.bits.contains($bit)
            }
            // pub fn [<effective_ $field>](&self, fallback: $ty) -> $ty where $ty: Copy {
            //     if self.bits.contains($bit) {
            //         self.$field
            //     } else {
            //         fallback
            //     }
            // }
        }
    };
}


impl StyleContext {
    pub fn cascade_from(&mut self, parent: &StyleContext) {
        for &bit in INHERITABLE_STYLES {
            if !self.bits.contains(bit) && parent.bits.contains(bit) {
                match bit {
                    StyleBits::COLOR => self.color = parent.color,
                    StyleBits::FONT_FAMILY => self.font_family = parent.font_family.clone(),
                    StyleBits::FONT_SIZE => self.font_size = parent.font_size,
                    StyleBits::FONT_STYLE => self.font_style = parent.font_style,
                    StyleBits::FONT_WEIGHT => self.font_weight = parent.font_weight,
                    StyleBits::LETTER_SPACING => self.letter_spacing = parent.letter_spacing,
                    StyleBits::LINE_HEIGHT => self.line_height = parent.line_height,
                    StyleBits::TEXT_ALIGN => self.text_align = parent.text_align,
                    StyleBits::TEXT_DECORATION => self.text_decoration = parent.text_decoration,
                    StyleBits::TEXT_TRANSFORM => self.text_transform = parent.text_transform,
                    StyleBits::WHITE_SPACE => self.white_space = parent.white_space,
                    StyleBits::WORD_SPACING => self.word_spacing = parent.word_spacing,
                    _ => {}
                }

                self.bits.insert(bit);
            }
        }

        //Enforce auto display rules
        if !self.has_display() {
            match parent.display {
                Display::Inline | Display::Block | Display::InlineBlock => {
                    self.set_display(Display::Inline);
                }
                _ => {}
            }
        }
    }
}


impl StyleContext {
    style_field!(display: Display, StyleBits::DISPLAY, Display::Block);
    style_field!(white_space: WhiteSpace, StyleBits::WHITE_SPACE, WhiteSpace::Normal);
    style_field!(opacity: f32, StyleBits::OPACITY, 1.0);

    style_field!(margin: Dimension, StyleBits::MARGIN, Dimension::default());
    style_field!(margin_top: Dimension, StyleBits::MARGIN_TOP, Dimension::default());
    style_field!(margin_bottom: Dimension, StyleBits::MARGIN_BOTTOM, Dimension::default());
    style_field!(margin_left: Dimension, StyleBits::MARGIN_LEFT, Dimension::default());
    style_field!(margin_right: Dimension, StyleBits::MARGIN_RIGHT, Dimension::default());

    style_field!(padding: Dimension, StyleBits::PADDING, Dimension::default());
    style_field!(padding_top: Dimension, StyleBits::PADDING_TOP, Dimension::default());
    style_field!(padding_bottom: Dimension, StyleBits::PADDING_BOTTOM, Dimension::default());
    style_field!(padding_left: Dimension, StyleBits::PADDING_LEFT, Dimension::default());
    style_field!(padding_right: Dimension, StyleBits::PADDING_RIGHT, Dimension::default());

    style_field!(align_content: AlignContent, StyleBits::ALIGN_CONTENT, AlignContent::FlexStart);
    style_field!(align_items: AlignItems, StyleBits::ALIGN_ITEMS, AlignItems::FlexStart);
    style_field!(align_self: AlignSelf, StyleBits::ALIGN_SELF, AlignSelf::Auto);
    style_field!(gap: Dimension, StyleBits::GAP, Dimension::default());
    style_field!(column_gap: Dimension, StyleBits::COLUMN_GAP, Dimension::default());
    style_field!(row_gap: Dimension, StyleBits::ROW_GAP, Dimension::default());

    style_field!(flex_basis: Length, StyleBits::FLEX_BASIS, Length::Auto);
    style_field!(flex_direction: FlexDirection, StyleBits::FLEX_DIRECTION, FlexDirection::Row);
    style_field!(flex_grow: f32, StyleBits::FLEX_GROW, 0.0);
    style_field!(flex_shrink: f32, StyleBits::FLEX_SHRINK, 0.0);
    style_field!(justify_content: JustifyContent, StyleBits::JUSTIFY_CONTENT, JustifyContent::FlexStart);
    style_field!(flex_wrap: FlexWrap, StyleBits::FLEX_WRAP, FlexWrap::NoWrap);

    style_field!(width: Dimension, StyleBits::WIDTH, Dimension::default());
    style_field!(max_width: Dimension, StyleBits::MAX_WIDTH, Dimension::default());
    style_field!(min_width: Dimension, StyleBits::MIN_WIDTH, Dimension::default());
    style_field!(height: Dimension, StyleBits::HEIGHT, Dimension::default());
    style_field!(max_height: Dimension, StyleBits::MAX_HEIGHT, Dimension::default());
    style_field!(min_height: Dimension, StyleBits::MIN_HEIGHT, Dimension::default());

    style_field!(text_align: TextAlign, StyleBits::TEXT_ALIGN, TextAlign::Left);
    style_field!(color: Color, StyleBits::COLOR, Color(0, 0, 0, 255));
    style_field!(text_decoration: TextDecoration, StyleBits::TEXT_DECORATION, TextDecoration::None);
    style_field!(font_family: FontFamily, StyleBits::FONT_FAMILY, FontFamily::Default);
    style_field!(font_size: Dimension, StyleBits::FONT_SIZE, Dimension::Default);
    style_field!(font_style: FontStyle, StyleBits::FONT_STYLE, FontStyle::Normal);
    style_field!(text_transform: TextTransform, StyleBits::TEXT_TRANSFORM, TextTransform::None);
    style_field!(letter_spacing: Dimension, StyleBits::LETTER_SPACING, Dimension::default());
    style_field!(line_height: Dimension, StyleBits::LINE_HEIGHT, Dimension::default());
    style_field!(font_weight: FontWeight, StyleBits::FONT_WEIGHT, FontWeight::Normal);
    style_field!(word_spacing: Dimension, StyleBits::WORD_SPACING, Dimension::default());

    style_field!(bg_color: Color, StyleBits::BG_COLOR, Color(255, 255, 255, 255));
    style_field!(bg_image: Image, StyleBits::BG_IMAGE, Image::default());
    style_field!(bg_position: BgPosition, StyleBits::BG_POSITION, BgPosition::Center);
    style_field!(bg_repeat: BgRepeat, StyleBits::BG_REPEAT, BgRepeat::NoRepeat);
    style_field!(bg_size: BgSize, StyleBits::BG_SIZE, BgSize::Contain);

    style_field!(border_radius: Dimension, StyleBits::BORDER_RADIUS, Dimension::default());
    style_field!(border_top_left_radius: Dimension, StyleBits::BORDER_TOP_LEFT, Dimension::default());
    style_field!(border_top_right_radius: Dimension, StyleBits::BORDER_TOP_RIGHT, Dimension::default());
    style_field!(border_bottom_left_radius: Dimension, StyleBits::BORDER_BOTTOM_LEFT, Dimension::default());
    style_field!(border_bottom_right_radius: Dimension, StyleBits::BORDER_BOTTOM_RIGHT, Dimension::default());
    style_field!(border_color: Color, StyleBits::BORDER_COLOR, Color(255, 255, 255, 255));
    style_field!(border_style: BorderStyle, StyleBits::BORDER_STYLE, BorderStyle::Solid);
    style_field!(border_width: Dimension, StyleBits::BORDER_WIDTH, Dimension::default());
}


impl Default for StyleContext {
    fn default() -> Self {
        Self {
            bits: Default::default(),
            display: Default::default(),
            white_space: Default::default(),
            opacity: 1.0,
            margin: Default::default(),
            margin_top: Default::default(),
            margin_bottom: Default::default(),
            margin_left: Default::default(),
            margin_right: Default::default(),
            padding: Default::default(),
            padding_top: Default::default(),
            padding_bottom: Default::default(),
            padding_left: Default::default(),
            padding_right: Default::default(),
            align_content: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            gap: Default::default(),
            column_gap: Default::default(),
            row_gap: Default::default(),
            flex_basis: Default::default(),
            flex_direction: Default::default(),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            justify_content: Default::default(),
            flex_wrap: Default::default(),
            width: Default::default(),
            max_width: Default::default(),
            min_width: Default::default(),
            height: Default::default(),
            max_height: Default::default(),
            min_height: Default::default(),
            text_align: Default::default(),
            color: Color(0,0,0,255),
            text_decoration: Default::default(),
            font_family: FontFamily::SansSerif,
            font_size: Default::default(),
            font_style: Default::default(),
            text_transform: Default::default(),
            letter_spacing: Default::default(),
            line_height: Default::default(),
            font_weight: Default::default(),
            word_spacing: Default::default(),
            bg_color: Color(255,255,255,255),
            bg_image: Image::None,
            bg_position: Default::default(),
            bg_repeat: Default::default(),
            bg_size: Default::default(),
            border_radius: Default::default(),
            border_top_left_radius: Default::default(),
            border_top_right_radius: Default::default(),
            border_bottom_left_radius: Default::default(),
            border_bottom_right_radius: Default::default(),
            border_color: Color(255,255,255,255),
            border_style: Default::default(),
            border_width: Default::default(),
        }
    }



}