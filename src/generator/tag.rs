use crate::{
    generator::Generator,
    parser::{Item, ItemC},
};

/// Tag of a [Element][crate::parser::Item::Element] after tag conversion stage.
///
/// Fg/Bg/Modifier variant is so-called builtin tag, Custom variant contains custom tag type.
#[derive(Debug, Clone)]
pub enum Tag<'a, C: TagConvertor<'a> + ?Sized> {
    /// Tag for change foreground color.
    Fg(C::Color),
    /// Tag for change background color.
    Bg(C::Color),
    /// Tag for use style modifier.
    Modifier(C::Modifier),
    /// A custom tag.
    Custom(C::Custom),
}

/// Tag type for a generator G.
pub type TagG<'a, G> = Tag<'a, <G as Generator<'a>>::Convertor>;

/// Trait for convert a raw tag string to [`Tag`] type.
///
/// Each generator has it own tag convertor, because different backend(show the final output)
/// supports different kind of tags.
///
/// This Trait has three assoc type:
///
/// - Color: for foreground or background color
/// - Modifier: for style modifier(like bold, italic, etc.)
/// - Custom: for custom tag
///
/// The Generator with this convertor `C` will received a series of Item<[Tag<C>][Tag]>, and convert it into final output.
pub trait TagConvertor<'a> {
    /// Color type for foreground and background typed tag.
    type Color;
    /// Modifier type for modifier typed tag.
    type Modifier;
    /// Custom tag type. Usually is the final type represent a style, can be converted from Color and Modifier.
    type Custom;

    /// Parse string to color type.
    fn parse_color(&mut self, s: &str) -> Option<Self::Color>;

    /// Parse string to modifier type.
    fn parse_modifier(&mut self, s: &str) -> Option<Self::Modifier>;

    /// Parse string to custom type.
    ///
    /// Only if this call fails, a convertor try to parse the raw tag string to a built-in tag.
    /// So the custom tag always have higher priority.
    fn parse_custom_tag(&mut self, s: &str) -> Option<Self::Custom>;

    /// Parse string to a builtin tag type.
    fn parse_built_in_tag(&mut self, s: &str) -> Option<Tag<'a, Self>> {
        let mut ty_value = s.split(':');
        let mut ty = ty_value.next()?;
        let value = ty_value.next().unwrap_or_else(|| {
            let value = ty;
            ty = "";
            value
        });

        if ty_value.next().is_some() {
            return None;
        }

        Some(match ty {
            "fg" => Tag::Fg(self.parse_color(value)?),
            "bg" => Tag::Bg(self.parse_color(value)?),
            "mod" => Tag::Modifier(self.parse_modifier(value)?),
            "" => {
                if let Some(color) = self.parse_color(value) {
                    Tag::Fg(color)
                } else if let Some(modifier) = self.parse_modifier(value) {
                    Tag::Modifier(modifier)
                } else {
                    return None;
                }
            }
            _ => return None,
        })
    }

    /// convert the tag string to [Tag] type
    fn convert_tag(&mut self, s: &'a str) -> Option<Tag<'a, Self>> {
        self.parse_custom_tag(s)
            .map(Tag::Custom)
            .or_else(|| self.parse_built_in_tag(s))
    }

    /// Convert item with raw tag string to item with [Tag] type.
    ///
    /// It will filtered out all tags that fail to parse.
    fn convert_item(&mut self, item: Item<'a>) -> ItemC<'a, Self> {
        match item {
            Item::PlainText(pt) => Item::PlainText(pt),
            Item::Element(spans, items) => {
                let tags = spans
                    .into_iter()
                    .filter_map(|span| self.convert_tag(span.fragment()))
                    .collect();

                let subitems = self.convert_line(items);

                Item::Element(tags, subitems)
            }
        }
    }

    /// Convert a line of items with raw tag string to items with [Tag] type.
    ///
    /// It will filtered out all tags that fail to parse.
    fn convert_line(&mut self, items: Vec<Item<'a>>) -> Vec<ItemC<'a, Self>> {
        items.into_iter().map(|item| self.convert_item(item)).collect()
    }

    /// Convert all item with raw tag string of a ast into items with [Tag] type.
    ///
    /// It will filtered out all tags that fail to parse.
    fn convert_ast(&mut self, ast: Vec<Vec<Item<'a>>>) -> Vec<Vec<ItemC<'a, Self>>> {
        ast.into_iter().map(|line| self.convert_line(line)).collect()
    }
}
