use std::collections::HashMap;

use omega_rsdd::fol::make_symbol_table;
fn main() {
    println!("Hello, world!");
	let x = "x";
	let range:(u64,u64) = (1,16);
	
	let mut temp_map: HashMap<String, (u64,u64)> = HashMap::new();
	temp_map.insert(x.to_string(), range);
	make_symbol_table(&temp_map);

}
