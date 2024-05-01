use std::collections::HashMap;
use rsdd::repr::BddPtr;

pub struct Context<'a> {
	Bdd: BddPtr<'a>,
	Op: HashMap<String,String>,
	Op_Bdd: HashMap<String,String>,
	vars : HashMap<String,String>,
}


pub fn make_symbol_table(vars: &HashMap<String, (u64,u64)>){
	let mut table: HashMap<String, Vec<u64>>;
println!("{:?}", vars );
	// vars.iter().
}
