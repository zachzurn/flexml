use bitflags::bitflags;
use paste::paste;

const STANDARD_DPI: f32 = 1.0 / 96.0;
const POINT_DPI: f32 = 1.0 / 72.0;
const MM_PER_INCH: f32 = 25.4;
const INCHES_PER_MM: f32 = 1.0 / MM_PER_INCH;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Dimension {
    #[default]
    Auto,
    Content,
    Zero,
    Px(f32),
    Percent(f32),
    Point(f32),
    Inch(f32),
    Mm(f32),
    Em(f32),
    Rem(f32),
    Resolved(f32)
}

impl Dimension {
    /// Calculate actual pixel sizes
    /// Auto and content will always produce 0
    /// If you are checking for auto or content
    /// you should match against that first
    pub fn to_pixels(&self, dim_px: f32, rem_px: f32, em_px: f32, dpi: f32) -> f32 {
        match self {
            Dimension::Percent(pct) => dim_px * pct,
            Dimension::Point(pt) => pt * (dpi * POINT_DPI),
            Dimension::Inch(inch) => inch * dpi,
            Dimension::Mm(mm) => mm * (dpi * INCHES_PER_MM),
            Dimension::Px(px) => px * (dpi * STANDARD_DPI),
            Dimension::Em(em) => em * em_px,
            Dimension::Rem(rem) => rem * rem_px,
            Dimension::Auto | Dimension::Content | Dimension::Zero => 0.0,
            Dimension::Resolved(resolved_value) => *resolved_value,
        }
    }

    /// Does the dimension have a non specified dimension
    /// like auto or content
    pub fn is_none(&self) -> bool {
        matches!(self, Dimension::Auto | Dimension::Content)
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


impl Color {
    fn transparent() -> Self {
        Self(0, 0, 0, 0)
    }
}



bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq)]
    pub struct StyleBits: u64 {

        const IS_ROOT              = 1 << 0;

        const DISPLAY              = 1 << 1;
        const WHITE_SPACE          = 1 << 2;
        const OPACITY              = 1 << 3;

        const MARGIN_TOP          = 1 << 5;
        const MARGIN_BOTTOM       = 1 << 6;
        const MARGIN_LEFT         = 1 << 7;
        const MARGIN_RIGHT        = 1 << 8;

        const PADDING_TOP         = 1 << 10;
        const PADDING_BOTTOM      = 1 << 11;
        const PADDING_LEFT        = 1 << 12;
        const PADDING_RIGHT       = 1 << 13;

        const ALIGN_CONTENT        = 1 << 14;
        const ALIGN_ITEMS          = 1 << 15;
        const ALIGN_SELF           = 1 << 16;
        const GAP                  = 1 << 17;
        const COLUMN_GAP           = 1 << 18;
        const ROW_GAP              = 1 << 19;
        const FLEX_BASIS           = 1 << 20;
        const FLEX_DIRECTION       = 1 << 21;
        const FLEX_GROW            = 1 << 22;
        const FLEX_SHRINK          = 1 << 23;
        const JUSTIFY_CONTENT      = 1 << 24;
        const FLEX_WRAP            = 1 << 25;

        const WIDTH                = 1 << 26;
        const MAX_WIDTH            = 1 << 27;
        const MIN_WIDTH            = 1 << 28;
        const HEIGHT               = 1 << 29;
        const MAX_HEIGHT           = 1 << 30;
        const MIN_HEIGHT           = 1 << 31;

        const TEXT_ALIGN           = 1 << 32;
        const COLOR                = 1 << 33;
        const TEXT_DECORATION      = 1 << 34;
        const FONT_FAMILY          = 1 << 35;
        const FONT_SIZE            = 1 << 36;
        const FONT_STYLE           = 1 << 37;
        const TEXT_TRANSFORM       = 1 << 38;
        const LETTER_SPACING       = 1 << 39;
        const LINE_HEIGHT          = 1 << 40;
        const FONT_WEIGHT          = 1 << 41;
        const WORD_SPACING         = 1 << 42;

        const BG_COLOR             = 1 << 43;
        const BG_IMAGE             = 1 << 44;
        const BG_POSITION          = 1 << 45;
        const BG_REPEAT            = 1 << 46;
        const BG_SIZE              = 1 << 47;

        const BORDER_TOP_LEFT      = 1 << 49;
        const BORDER_TOP_RIGHT     = 1 << 50;
        const BORDER_BOTTOM_LEFT   = 1 << 51;
        const BORDER_BOTTOM_RIGHT  = 1 << 52;
        const BORDER_COLOR         = 1 << 53;
        const BORDER_STYLE         = 1 << 54;
        const BORDER_WIDTH         = 1 << 55;
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

    pub dpi: f32,

    pub display: Display,
    pub white_space: WhiteSpace,
    pub opacity: f32,

    pub margin_top: Dimension,
    pub margin_bottom: Dimension,
    pub margin_left: Dimension,
    pub margin_right: Dimension,

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

    pub flex_basis: Dimension,
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
    pub resolved_font_size: f32,
    pub resolved_root_font_size: f32,
    pub font_style: FontStyle,
    pub text_transform: TextTransform,
    pub letter_spacing: Dimension,
    pub line_height: Dimension,
    pub font_weight: u16,
    pub word_spacing: Dimension,

    pub bg_color: Color,
    pub bg_image: Image,
    pub bg_position: BgPosition,
    pub bg_repeat: BgRepeat,
    pub bg_size: BgSize,

    pub border_top_left_radius: Dimension,
    pub border_top_right_radius: Dimension,
    pub border_bottom_left_radius: Dimension,
    pub border_bottom_right_radius: Dimension,

    pub border_color: Color,
    pub border_style: BorderStyle,
    pub border_width: Dimension,
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
        if !self.has_display() && !parent.is_root() {
            match parent.display {
                Display::Inline | Display::Block | Display::InlineBlock => {
                    self.set_display(Display::Inline);
                }
                _ => {}
            }
        }

        // Calculate resolved font sizes
        self.resolved_font_size = self.font_size.to_pixels(
            parent.resolved_font_size,       // for Percent and Em
            parent.resolved_root_font_size,  // for Rem
            parent.resolved_font_size,       // for Em again
            parent.dpi,
        );

        // Propagate root font size (unchanged from parent)
        self.resolved_root_font_size = parent.resolved_root_font_size;
    }
}

macro_rules! style_field {
    ($field:ident : $ty:ty, $bit:expr) => {
        paste! {
            pub fn [<set_ $field>](&mut self, value: $ty) {
                self.$field = value;
                self.bits.insert($bit);
            }

            pub fn [<has_ $field>](&self) -> bool {
                self.bits.contains($bit)
            }
        }
    };
}

impl StyleContext {

    pub fn set_as_root(&mut self) {
        self.bits.insert(StyleBits::IS_ROOT);
    }

    pub fn is_root(&self) -> bool {
        self.bits.contains(StyleBits::IS_ROOT)
    }

    pub fn default_font_size_pixels() -> f32 {
        16.0f32
    }

    pub fn min_font_size_pixels() -> f32 {
        1.0f32
    }

    pub fn default_page_width_resolved() -> f32 {
        1920.0f32
    }

    pub fn min_page_width_resolved() -> f32 {
        50.0f32
    }

    pub fn default_page_height_resolved() -> f32 {
        1080.0f32
    }

    pub fn min_page_height_resolved() -> f32 {
        50.0f32
    }

    pub fn default_dpi() -> f32 {
        72.0f32
    }

    pub fn min_dpi() -> f32 {
        25.0f32
    }

    style_field!(display: Display, StyleBits::DISPLAY);
    style_field!(white_space: WhiteSpace, StyleBits::WHITE_SPACE);
    style_field!(opacity: f32, StyleBits::OPACITY);

    style_field!(margin_top: Dimension, StyleBits::MARGIN_TOP);
    style_field!(margin_bottom: Dimension, StyleBits::MARGIN_BOTTOM);
    style_field!(margin_left: Dimension, StyleBits::MARGIN_LEFT);
    style_field!(margin_right: Dimension, StyleBits::MARGIN_RIGHT);

    style_field!(padding_top: Dimension, StyleBits::PADDING_TOP);
    style_field!(padding_bottom: Dimension, StyleBits::PADDING_BOTTOM);
    style_field!(padding_left: Dimension, StyleBits::PADDING_LEFT);
    style_field!(padding_right: Dimension, StyleBits::PADDING_RIGHT);

    style_field!(align_content: AlignContent, StyleBits::ALIGN_CONTENT);
    style_field!(align_items: AlignItems, StyleBits::ALIGN_ITEMS);
    style_field!(align_self: AlignSelf, StyleBits::ALIGN_SELF);
    style_field!(gap: Dimension, StyleBits::GAP);
    style_field!(column_gap: Dimension, StyleBits::COLUMN_GAP);
    style_field!(row_gap: Dimension, StyleBits::ROW_GAP);

    style_field!(flex_basis: Dimension, StyleBits::FLEX_BASIS);
    style_field!(flex_direction: FlexDirection, StyleBits::FLEX_DIRECTION);
    style_field!(flex_grow: f32, StyleBits::FLEX_GROW);
    style_field!(flex_shrink: f32, StyleBits::FLEX_SHRINK);
    style_field!(justify_content: JustifyContent, StyleBits::JUSTIFY_CONTENT);
    style_field!(flex_wrap: FlexWrap, StyleBits::FLEX_WRAP);

    style_field!(width: Dimension, StyleBits::WIDTH);
    style_field!(max_width: Dimension, StyleBits::MAX_WIDTH);
    style_field!(min_width: Dimension, StyleBits::MIN_WIDTH);
    style_field!(height: Dimension, StyleBits::HEIGHT);
    style_field!(max_height: Dimension, StyleBits::MAX_HEIGHT);
    style_field!(min_height: Dimension, StyleBits::MIN_HEIGHT);

    style_field!(text_align: TextAlign, StyleBits::TEXT_ALIGN);
    style_field!(color: Color, StyleBits::COLOR);
    style_field!(text_decoration: TextDecoration, StyleBits::TEXT_DECORATION);
    style_field!(font_family: FontFamily, StyleBits::FONT_FAMILY);
    style_field!(font_size: Dimension, StyleBits::FONT_SIZE);
    style_field!(font_style: FontStyle, StyleBits::FONT_STYLE);
    style_field!(text_transform: TextTransform, StyleBits::TEXT_TRANSFORM);
    style_field!(letter_spacing: Dimension, StyleBits::LETTER_SPACING);
    style_field!(line_height: Dimension, StyleBits::LINE_HEIGHT);
    style_field!(font_weight: u16, StyleBits::FONT_WEIGHT);
    style_field!(word_spacing: Dimension, StyleBits::WORD_SPACING);

    style_field!(bg_color: Color, StyleBits::BG_COLOR);
    style_field!(bg_image: Image, StyleBits::BG_IMAGE);
    style_field!(bg_position: BgPosition, StyleBits::BG_POSITION);
    style_field!(bg_repeat: BgRepeat, StyleBits::BG_REPEAT);
    style_field!(bg_size: BgSize, StyleBits::BG_SIZE);

    style_field!(border_top_left_radius: Dimension, StyleBits::BORDER_TOP_LEFT);
    style_field!(border_top_right_radius: Dimension, StyleBits::BORDER_TOP_RIGHT);
    style_field!(border_bottom_left_radius: Dimension, StyleBits::BORDER_BOTTOM_LEFT);
    style_field!(border_bottom_right_radius: Dimension, StyleBits::BORDER_BOTTOM_RIGHT);

    style_field!(border_color: Color, StyleBits::BORDER_COLOR);
    style_field!(border_style: BorderStyle, StyleBits::BORDER_STYLE);
    style_field!(border_width: Dimension, StyleBits::BORDER_WIDTH);
}


impl Default for StyleContext {
    fn default() -> Self {
        Self {
            dpi: 160.0f32,

            bits: Default::default(),
            display: Default::default(),
            white_space: Default::default(),
            opacity: 1.0,
            margin_top: Dimension::Zero,
            margin_bottom: Dimension::Zero,
            margin_left: Dimension::Zero,
            margin_right: Dimension::Zero,

            padding_top: Dimension::Zero,
            padding_bottom: Dimension::Zero,
            padding_left: Dimension::Zero,
            padding_right: Dimension::Zero,
            align_content: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            gap: Dimension::Zero,
            column_gap: Dimension::Zero,
            row_gap: Dimension::Zero,
            flex_basis: Default::default(),
            flex_direction: Default::default(),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            justify_content: Default::default(),
            flex_wrap: Default::default(),
            width: Dimension::Inch(8.5),
            max_width: Default::default(),
            min_width: Default::default(),
            height: Dimension::Inch(11.0),
            max_height: Default::default(),
            min_height: Default::default(),
            text_align: TextAlign::Left,
            color: Color(0,0,0,255),
            text_decoration: Default::default(),
            font_family: FontFamily::SansSerif,
            font_size: Dimension::Px(StyleContext::default_font_size_pixels()),
            resolved_font_size: 0.0f32,
            resolved_root_font_size: 0.0f32,
            font_style: Default::default(),
            text_transform: Default::default(),
            letter_spacing: Default::default(),
            line_height: Default::default(),
            font_weight: 300,
            word_spacing: Default::default(),
            bg_color: Color::transparent(),
            bg_image: Image::None,
            bg_position: Default::default(),
            bg_repeat: Default::default(),
            bg_size: Default::default(),
            border_top_left_radius: Dimension::Zero,
            border_top_right_radius: Dimension::Zero,
            border_bottom_left_radius: Dimension::Zero,
            border_bottom_right_radius: Dimension::Zero,
            border_color: Color::transparent(),
            border_style: BorderStyle::Solid,
            border_width: Dimension::Zero,
        }
    }



}