use mona::attribute::SimpleAttributeGraph2;
use mona::character::character_common_data::CharacterCommonData;
use mona::enemies::Enemy;

pub struct Span {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl Span {
    pub fn from_pest_span(span: &pest::Span) -> Span {
        let (start_row, start_col) = span.start_pos().line_col();
        let (end_row, end_col) = span.end_pos().line_col();
        Span {
            start_row,
            start_col,
            end_row,
            end_col
        }
    }
}

pub struct UnsafeDamageContext {
    pub character_common_data: *const CharacterCommonData,
    pub enemy: *const Enemy,
    pub attribute: *const SimpleAttributeGraph2
}
