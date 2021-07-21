extern crate sexpr;
use sexpr::parsers::*;

fn main() {
    println!("0.result={:?}", parse_term_node("X"));

    println!("0.2.result={:?}", parse_term_node("X Y Z"));

    println!("0.25.result={:?}", parse_term_node("  X123YZ Y92 Z29 "));

    println!("0.5.result={:?}", parse_func_name("  ADD X Y)"));

    println!("1.result={:?}", parse_sexpr("Y"));

    println!("2.result={:?}", parse_sexpr("(ADD X Y)"));

    println!("3.0.result={:?}", parse_sexpr("( ADD X Y)"));

    println!("3.2.result={:?}", parse_sexpr("( ADD  X Y)"));

    println!("3.4.result={:?}", parse_sexpr("( ADD  X    Y)"));

    println!("3.6.result={:?}", parse_sexpr("( ADD  X    Y )"));

    println!("3.9.result={:?}", parse_sexpr("( ADD    X Y  )"));

    println!("3.95.result={:?}", parse_sexpr("( ADD ()   X Y  )"));

    println!("4.result={:?}", parse_sexpr("( ADD    X (DIV (IF 3 1 3)  2) ( MULT 1 4 1  )  )"));

    println!("5.result={:?}", skip_spaces("  \t \r  \n\n AABC DEF "));

    println!("6.result={:?}", parse_term_node(")  X123YZ Y92 Z29 "));

    println!("7.result={:?}", parse_func_node(" ( XYZ 1  2   (AD 3) ) "));
    println!("8.result={:?}", parse_func_node(" ( XYZ 1  2   (AD 3 ) "));
    println!("9.result={:?}", parse_func_name("    ABCD123 "));
    println!("10.result={:?}", parse_func_name(" (   ABCD123 )"));
    println!("11.result={:?}", parse_func_call("  ABCD123 X1 (Y 1) Z  "));
    println!("12.result={:?}", parse_func_call("  (ABCD123 X1 (Y 1) Z)  "));
    println!("13.result={:?}", parse_func_args("  X1 (Y 1) Z  "));
    println!("14.result={:?}", parse_func_args("  (X1 (Y 1) Z   "));
    println!("15.result={:?}", parse_func_args("  ((X1 Y) X) (Y 1) Z   "));
}
