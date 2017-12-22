use zeal::parser::{ErrorMessage, ParseNode};
use zeal::symbol_table::SymbolTable;

pub trait TreePass<'a> {
    fn has_errors(&self) -> bool;
    fn get_error_messages(&self) -> &Vec<ErrorMessage<'a>>;
    fn do_pass(&mut self, Vec<ParseNode<'a>>, &mut SymbolTable) -> Vec<ParseNode<'a>>;
}
