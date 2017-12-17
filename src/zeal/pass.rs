use zeal::parser::ParseNode;
use zeal::symbol_table::SymbolTable;

pub trait TreePass<'a> {
    fn has_errors(&self) -> bool;
    fn do_pass(&mut self, &Vec<ParseNode<'a>>, &mut SymbolTable) -> Vec<ParseNode<'a>>;
}
