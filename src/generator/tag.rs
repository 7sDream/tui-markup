use crate::{
    generator::Generator,
    parser::{Item, ItemC, LSpan},
};

/// Tag of a [Element][crate::parser::Item::Element] after transform step.
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

/// Trait for convert a raw tag string to tag type.
///
/// Each generator has it own tag generator, because different backend(show the final output)
/// supports different kind of tags.
///
/// So this tag generator should raise a error if there are unsupported tags in source code.
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

    /// Parse string to custom type,
    /// will be only called when a tag string isn't a valid and acceptable builtin tag.
    fn parse_custom_tag(&mut self, s: &str) -> Option<Self::Custom>;

    /// Parse string to a builtin tag type.
    fn parse_built_in_tag(&mut self, ty: &str, value: &str) -> Option<Tag<'a, Self>> {
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
    fn convert_tag(&mut self, s: LSpan<'a>) -> Option<Tag<'a, Self>> {
        let mut ty_value = s.split(':');
        let mut ty = ty_value.next()?;
        let value = ty_value.next().unwrap_or_else(|| {
            let value = ty;
            ty = "";
            value
        });

        if ty_value.next().is_some() {
            return Some(Tag::Custom(self.parse_custom_tag(s.fragment())?));
        }

        self.parse_custom_tag(s.fragment())
            .map(Tag::Custom)
            .or_else(|| self.parse_built_in_tag(ty, value))
    }

    /// Convert item with raw tag string to item with [Tag] type.
    fn convert_item(&mut self, item: Item<'a>) -> Result<ItemC<'a, Self>, LSpan<'a>> {
        match item {
            Item::PlainText(pt) => Ok(Item::PlainText(pt)),
            Item::Element(spans, items) => {
                let mut tags = Vec::with_capacity(spans.len());

                for span in spans {
                    let tag = self.convert_tag(span).ok_or(span)?;
                    tags.push(tag);
                }

                let subitems = self.convert_line(items)?;

                Ok(Item::Element(tags, subitems))
            }
        }
    }

    /// Convert a line of items with raw tag string to items with [Tag] type.
    fn convert_line(&mut self, items: Vec<Item<'a>>) -> Result<Vec<ItemC<'a, Self>>, LSpan<'a>> {
        items.into_iter().map(|item| self.convert_item(item)).collect()
    }

    /// Convert all item with raw tag string of a ast into items with [Tag] type.
    fn convert(&mut self, ast: Vec<Vec<Item<'a>>>) -> Result<Vec<Vec<ItemC<'a, Self>>>, LSpan<'a>> {
        ast.into_iter().map(|line| self.convert_line(line)).collect()
    }
}
