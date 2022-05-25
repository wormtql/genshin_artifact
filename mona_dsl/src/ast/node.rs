// use pest::Span;
//
// pub struct ASTNode<'i, T> {
//     pub span: Option<Span<'i>>,
//
//     pub data: T
// }
//
// impl<'i, T> ASTNode<'i, T> {
//     pub fn new(input: &'i str, data: T) -> ASTNode<'i, T> {
//         ASTNode {
//             span: Span::new(input, 0, 0),
//             data,
//         }
//     }
// }

use pest::Span;

#[derive(Clone)]
pub struct NodeCommon<'i> {
    pub input: &'i str,
    pub span: Span<'i>,
}

impl<'i> NodeCommon<'i> {
    pub fn merge_span(&self, other: &NodeCommon<'i>) -> NodeCommon<'i> {
        let start = self.span.start();
        let end = other.span.end();
        NodeCommon {
            input: self.input,
            span: Span::new(self.input, start, end).unwrap()
        }
    }

    pub fn merge_span2(&self, other: &Span<'i>) -> NodeCommon<'i> {
        let start = self.span.start();
        let end = other.end();
        NodeCommon {
            input: self.input,
            span: Span::new(self.input, start, end).unwrap()
        }
    }
}
