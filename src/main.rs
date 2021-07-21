extern crate sexpr;
use sexpr::parsers::*;

fn main() {
    println!("0.result={:?}", parse_term_node("X"));

    println!("0.5.result={:?}", parse_func_name("  ADD X Y)"));

    println!("1.result={:?}", parse_sexpr("Y"));

    println!("2.result={:?}", parse_sexpr("(ADD X Y)"));

    println!("3.0.result={:?}", parse_sexpr("( ADD X Y)"));

    println!("3.2.result={:?}", parse_sexpr("( ADD  X Y)"));

    println!("3.4.result={:?}", parse_sexpr("( ADD  X    Y)"));

    println!("3.6.result={:?}", parse_sexpr("( ADD  X    Y )"));

    println!("3.9.result={:?}", parse_sexpr("( ADD    X Y  )"));

    println!("4.result={:?}", parse_sexpr("( ADD    X (DIV (IF 3 1 3)  2) ( MULT 1 4 1  )  )"));
}
