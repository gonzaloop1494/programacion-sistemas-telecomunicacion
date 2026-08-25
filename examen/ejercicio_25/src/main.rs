fn next_token(s: &str, start: usize) -> Option<(usize, usize)> {
    let s = &s[start..]; //variable que me guarda una RODAJA desde la posición a la que punta START hasta el final del STRING
    let mut in_word = false; //Variable booleana que me dice si hay palabra o no
    let mut word_start = 0; //Desplazamiento desde el comienzo del String

        //pos del caracter y caracter en cuestión //char_indices() sobre un String-> devuelve Duplas formadas por un caracter y la pos que ocupa
    for (i, c) in s.char_indices() { //recorre la RODAJA de nuestro STRING los índices y los caracteres asociados a cada uno de esos índices
        if c == ' ' {  //si hay espacio en blanco
            if in_word {  // y está dentro de la  palabra
                return Some((word_start + start, i - 1 + start)); //acaba la palabra
            }
        } else {
            if !in_word { //if in_word == False -> si no estamos en una palabra
                in_word = true;
                word_start = i;
            }
        }
    }
    if in_word { // si me encuentro la palabra al final de mi cadena de caracteres
        Some((word_start + start, s.len() - 1 + start)) //Traducción entre el String original y nuestra rodaja
    } else {
        None //por ejemplo que me pongan el start en 10
    }
}


#[test]
fn t1(){
    assert_eq!(None, next_token(&"   ",0));
}

#[test]
fn t2(){
    assert_eq!((1,4), next_token(&" hola hola", 0).unwrap());
    assert_eq!((6,9), next_token(&" hola hola", 5).unwrap());
    assert_eq!(None , next_token(&" hola hola", 10));
}

#[test]
fn t3(){
    assert_eq!((0,3), next_token("hola ",0).unwrap());
    assert_eq!((2,5), next_token("  hola ", 0).unwrap());
    assert_eq!((1,4), next_token(" hola ", 0).unwrap());
    assert_eq!((1,4), next_token(" hola", 0).unwrap());
}

fn main() {}