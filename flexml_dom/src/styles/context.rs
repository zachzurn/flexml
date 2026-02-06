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
    pub fn as_pixels(&self, dim_px: f32, rem_px: f32, em_px: f32, dpi: f32) -> f32 {
        match self {
            Dimension::Percent(pct) => dim_px * pct,
            Dimension::Point(pt) => pt * (dpi * POINT_DPI),
            Dimension::Inch(inch) => inch * dpi,
            Dimension::Mm(mm) => mm * (dpi * INCHES_PER_MM),
            Dimension::Px(px) => px * (dpi * STANDARD_DPI),
            Dimension::Em(em) => em_px * em,
            Dimension::Rem(rem) => rem_px * rem,
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

        const DPI                  = 1 << 4;

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
/// All fields in StyleContext are private
/// It's important that any changes are tracked
/// with style bits
///
/// Use public getters and setters
pub struct StyleContext {
    bits: StyleBits,

    dpi: f32,

    display: Display,
    white_space: WhiteSpace,
    opacity: f32,

    margin_top: Dimension,
    margin_bottom: Dimension,
    margin_left: Dimension,
    margin_right: Dimension,

    padding_top: Dimension,
    padding_bottom: Dimension,
    padding_left: Dimension,
    padding_right: Dimension,

    align_content: AlignContent,
    align_items: AlignItems,
    align_self: AlignSelf,
    gap: Dimension,
    column_gap: Dimension,
    row_gap: Dimension,

    flex_basis: Dimension,
    flex_direction: FlexDirection,
    flex_grow: f32,
    flex_shrink: f32,
    justify_content: JustifyContent,
    flex_wrap: FlexWrap,

    width: Dimension,
    max_width: Dimension,
    min_width: Dimension,
    height: Dimension,
    max_height: Dimension,
    min_height: Dimension,

    text_align: TextAlign,
    color: Color,
    text_decoration: TextDecoration,
    font_family: FontFamily,
    font_size: Dimension,
    resolved_font_size: f32,
    resolved_root_font_size: f32,
    font_style: FontStyle,
    text_transform: TextTransform,
    letter_spacing: Dimension,
    line_height: Dimension,
    font_weight: u16,
    word_spacing: Dimension,

    bg_color: Color,
    bg_image: Image,
    bg_position: BgPosition,
    bg_repeat: BgRepeat,
    bg_size: BgSize,

    border_top_left_radius: Dimension,
    border_top_right_radius: Dimension,
    border_bottom_left_radius: Dimension,
    border_bottom_right_radius: Dimension,

    border_color: Color,
    border_style: BorderStyle,
    border_width: Dimension,
}

impl StyleContext {
    pub fn cascade_from(&mut self, parent: &StyleContext) {
        // TODO at some point figure out how to pull root only styles into
        // something else so that we don't have copies of every root parameter
        // on every style_context. Also we should maybe separate layout context
        // from text context

        // We check if the style was not explicitly set and then
        // set the style from the parent.
        for &bit in INHERITABLE_STYLES {
            if !self.bits.contains(bit) && parent.bits.contains(bit) {
                match bit {
                    StyleBits::COLOR => self.set_color(parent.color),
                    StyleBits::FONT_FAMILY => self.set_font_family(parent.font_family),
                    StyleBits::FONT_SIZE => self.set_font_size(parent.font_size),
                    StyleBits::FONT_STYLE => self.set_font_style(parent.font_style),
                    StyleBits::FONT_WEIGHT => self.set_font_weight(parent.font_weight),
                    StyleBits::LETTER_SPACING => self.set_letter_spacing(parent.letter_spacing),
                    StyleBits::LINE_HEIGHT => self.set_line_height(parent.line_height),
                    StyleBits::TEXT_ALIGN => self.set_text_align(parent.text_align),
                    StyleBits::TEXT_DECORATION => self.set_text_decoration(parent.text_decoration),
                    StyleBits::TEXT_TRANSFORM => self.set_text_transform(parent.text_transform),
                    StyleBits::WHITE_SPACE => self.set_white_space(parent.white_space),
                    StyleBits::WORD_SPACING => self.set_word_spacing(parent.word_spacing),
                    _ => {}
                }

                self.bits.insert(bit);
            }
        }

        //Enforce auto display rules
        //Anything inside a block, inline or inline block
        //That didn't explicitly set a Display will be set to inline
        if !self.has_display() && !parent.is_root() {
            match parent.display() {
                Display::Inline | Display::Block | Display::InlineBlock => {
                    self.set_display(Display::Inline);
                }
                _ => {}
            }
        }

        // Calculate resolved font sizes
        // This makes cascaded em and rem dimensions possible
        self.resolved_font_size = self.font_size.as_pixels(
            parent.resolved_font_size,       // for Percent and Em
            parent.resolved_root_font_size,  // for Rem
            parent.resolved_font_size,       // for Em again
            parent.dpi,
        );

        // Propagate root font size and dpi (unchanged from parent)
        self.dpi = parent.dpi;
        self.resolved_root_font_size = parent.resolved_root_font_size;
    }

    pub fn prepare_root(&mut self) {
        // We need to ensure the fundamental dimensions are sound
        let default_dpi = 160.0;
        let default_font_size_pixels = 16.0;
        let default_page_width_inches = 8.5;
        let default_page_height_inches = 11.0;
        let default_page_margin_inches = 0.25;

        // Minimums for no particular reason, but we don't want 0
        let min_dpi: f32 = 100.0;
        let min_page_dimension_resolved: f32 = 50.0;
        let min_font_size_resolved: f32 = 6.0;

        // Default page color
        if !self.has_bg_color() {
            self.set_bg_color(Color(255,255,255,255));
        }

        // Set default margins if not set by user
        if !self.has_padding_top() { self.set_padding_top(Dimension::Inch(default_page_margin_inches)) }
        if !self.has_padding_left() { self.set_padding_left(Dimension::Inch(default_page_margin_inches)) }
        if !self.has_padding_bottom() { self.set_padding_bottom(Dimension::Inch(default_page_margin_inches)) }
        if !self.has_padding_right() { self.set_padding_right(Dimension::Inch(default_page_margin_inches)) }

        // Set default page dimensions if not set by user
        if !self.has_width() { self.set_width(Dimension::Inch(default_page_width_inches)) }
        if !self.has_height() { self.set_height(Dimension::Inch(default_page_height_inches)) }

        // Default font size if not user set
        if !self.has_font_size() { self.set_font_size(Dimension::Px(default_font_size_pixels)) }

        // DPI must be set first
        if !self.has_dpi() { self.set_dpi(default_dpi) }

        // Ensure DPI is set to minimum
        self.set_dpi(min_dpi.max(self.dpi()).round());

        // Set resolved font sizes. For em and rem we use the default font size, so 1em or 1rem = default font size
        let default_font_size_resolved = default_font_size_pixels * self.dpi();
        self.set_resolved_font_size(min_font_size_resolved.max(self.font_size().as_pixels(default_font_size_resolved, default_font_size_resolved, default_font_size_resolved, self.dpi())));
        self.set_resolved_root_font_size(self.resolved_font_size());

        // Set resolved page dimensions
        // 50% for example would resolve to 50% of the default page width
        let default_page_width_resolved = default_page_width_inches * self.dpi();
        let default_page_height_resolved = default_page_height_inches * self.dpi();
        self.set_width(Dimension::Resolved(min_page_dimension_resolved.max(self.width().as_pixels(default_page_width_resolved, self.resolved_root_font_size(), self.resolved_font_size(), self.dpi())).round()));
        self.set_height(Dimension::Resolved(min_page_dimension_resolved.max(self.height().as_pixels(default_page_height_resolved, self.resolved_root_font_size(), self.resolved_font_size(), self.dpi())).round()));
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

            pub fn [<$field>](&self) -> $ty {
                self.$field
            }
        }
    };
}

impl StyleContext {

    pub fn set_as_root(&mut self) {
        self.bits.insert(StyleBits::IS_ROOT);
    }

    pub fn clear_as_root(&mut self) {
        self.bits.remove(StyleBits::IS_ROOT);
    }

    pub fn is_root(&self) -> bool {
        self.bits.contains(StyleBits::IS_ROOT)
    }

    pub fn resolved_font_size(&self) -> f32{
        self.resolved_font_size
    }

    pub fn set_resolved_font_size(&mut self, resolved_font_size: f32) {
        self.resolved_font_size = resolved_font_size
    }

    pub fn resolved_root_font_size(&self) -> f32{
        self.resolved_root_font_size
    }

    pub fn set_resolved_root_font_size(&mut self, resolved_root_font_size: f32) {
        self.resolved_root_font_size = resolved_root_font_size
    }

    style_field!(dpi: f32, StyleBits::DPI);

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
            dpi: 0.0, // cascaded from root
            resolved_font_size: 0.0f32, // cascaded from parent
            resolved_root_font_size: 0.0f32, // cascaded from root

            bits: Default::default(),
            display: Display::Block,
            white_space: WhiteSpace::Normal,
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
            width: Dimension::Auto,
            max_width: Dimension::Auto,
            min_width: Dimension::Auto,
            height: Dimension::Auto,
            max_height: Dimension::Auto,
            min_height: Dimension::Auto,
            text_align: TextAlign::Left,
            color: Color(0,0,0,255),
            text_decoration: Default::default(),
            font_family: FontFamily::SansSerif,
            font_size: Dimension::Zero, //This will cascade from root
            font_style: Default::default(),
            text_transform: Default::default(),
            letter_spacing: Default::default(),
            line_height: Dimension::Em(1.2),
            font_weight: 400,
            word_spacing: Default::default(),
            bg_color: Color::transparent(),
            bg_image: Image::None,
            bg_position: Default::default(),
            bg_repeat: Default::default(),
            bg_size: Default::default(),

            //TODO we need other border properties separated out
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

impl core::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dimension::Auto => write!(f, "auto"),
            Dimension::Px(v) => write!(f, "{}px", v),
            Dimension::Percent(v) => write!(f, "{}%", v),
            Dimension::Zero => write!(f, "0"),
            Dimension::Inch(v) => write!(f, "{}in", v),
            Dimension::Em(v) => write!(f, "{}em", v),
            Dimension::Content => write!(f, "content"),
            Dimension::Mm(v) => write!(f, "{}mm", v),
            Dimension::Point(v) => write!(f, "{}pt", v),
            Dimension::Rem(v) => write!(f, "{}rem", v),
            Dimension::Resolved(v) => write!(f, "{}(resolved)", v),
        }
    }
}