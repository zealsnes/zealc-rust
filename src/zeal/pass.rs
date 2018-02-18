use zeal::parser::{ErrorMessage, ParseNode};
use zeal::symbol_table::SymbolTable;

pub trait TreePass {
    fn has_errors(&self) -> bool;
    fn get_error_messages(&self) -> &Vec<ErrorMessage>;
    fn do_pass(&mut self, Vec<ParseNode>, &mut SymbolTable) -> Vec<ParseNode>;
}
