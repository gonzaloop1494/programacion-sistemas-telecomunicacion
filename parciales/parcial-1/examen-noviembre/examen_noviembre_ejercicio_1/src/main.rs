mod pila;
mod tokens;


use pila::Pila;
use tokens::next_token;


fn parentesis_equilibrados(s: &str) -> bool {
    let mut parentesis: Pila<&str, 25> = Pila::new();
    let mut llaves : Pila<&str, 25> = Pila::new();

    let mut start: usize = 0;
    while let Some((begin, end)) = next_token(s, start) {
        let token = &s[begin..end];
        match token {
            "(" => {
                parentesis.push(token).unwrap();
            },
            ")" => {
                let Ok(_) = parentesis.pop() else {
                    return false;
                };
            },
            "{" => {
                llaves.push(token).unwrap();
            },
            _ => { /* "}" => { ... } */
                let Ok(_) = llaves.pop() else {
                    return false;
                };
            }
        }
        start = end + 1;
    }
    parentesis.is_empty() && llaves.is_empty()
}






#[test]
fn test_1() {
    assert_eq!(true, parentesis_equilibrados(&" ( )   "));
}

#[test]
fn test_2() {
    assert_eq!(true, parentesis_equilibrados(&" ( ) {      } "));
}

#[test]
fn test_3() {
    assert_eq!(true, parentesis_equilibrados(&" {     ( }   )  "));
}

#[test]
fn test_4() {
    assert_eq!(false, parentesis_equilibrados(&"}     () {  "));
}

#[test]
fn test_5() {
    assert_eq!(false, parentesis_equilibrados(&"} (  "));
}

#[test]
fn test_6() {
    assert_eq!(
        false,
        parentesis_equilibrados(&"( ( {    (  ) }     {   )    } )  )    ")
    );
}

fn main() {}