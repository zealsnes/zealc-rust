use zeal::parser::ParseNode;

pub trait TreePass<'a> {
    fn has_errors(&self) -> bool;
    fn do_pass(&mut self, &Vec<ParseNode<'a>>) -> Vec<ParseNode<'a>>;
}
