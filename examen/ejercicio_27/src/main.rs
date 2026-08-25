//Aquí se gestiona el flujo de operación, centrado en la
//evaluación de expresiones matemáticas

//Se importan las estructuras y funciones de los módulos:
//cola, pila, tokens que se utilizan en la evaluación de las expresiones
use cola::Cola;
use pila::Pila;
use tokens::next_token;

//se incluyen los módulos tokens, pila, cola, read_file
//para que Rust los reconozca como partes del proyecto
mod tokens;
mod cola;
mod pila;
mod read_file;

//constante que define un mensaje de error genérico
const ERR_MSG: &str = "Incorrect expression";

//esta función aplica un operador específico sobre operandos extraídos de la
//pila operands
fn apply_operator(operator: &str, operands: &mut Pila<f64, 25>) -> Result<(), String> {
    let Ok(right_operand) = operands.pop() else { //intenta extraer el último valor de la pila operands
        return Err(String::from(ERR_MSG));
    };
    if operator == "sqrt" {
        if operands.push(right_operand.sqrt()).is_err() { //calcula la raiz cuadrada del operando derecho
                                                          //y lo vuelve a colocar de nuevo en la pila
            return Err(String::from(ERR_MSG)); //si push falla retorna un error
        }
    } else {
        let Ok(left_operand) = operands.pop() else {
            return Err(String::from(ERR_MSG));
        };
        let result: f64 = match operator { //operadores binarios
            "+" => left_operand + right_operand, //extrae un segundo valor de la pila
            "-" => left_operand - right_operand,
            "*" => left_operand * right_operand,
            "/" => {
                if right_operand == 0.0 {
                    return Err(String::from(ERR_MSG));
                }
                left_operand / right_operand
            }
            _ => { return Err(String::from(ERR_MSG)); }
        };
        if operands.push(result).is_err() {
            return Err(String::from(ERR_MSG));
        }
    }
    Ok(()) //inserta el resultado en la pila
}
//evalúa la expresión matemática en formato de cadena utilizando
//una cola y dos pilas
fn eval(e: String) -> Result<f64, String> {
    let mut tokens: Cola<&str, 50> = Cola::new(); //usa next_token para extraer tokens
    //operadores y operandos, y los coloca en una cola
    let mut start = 0; //apunta a una dirección inicializada a 0
    while let Some((begin, end)) = next_token(&e, start) { //si enqueue falla, retorna error
        if tokens.enqueue(&e[begin..=end]).is_err() { //indica una cola llena o problema de expresión
            return Err(String::from(ERR_MSG));
        }
        start = end + 1; //incrementa a la siguiente posición el "puntero"
    }


    let mut operands: Pila<f64, 25> = Pila::new(); //pila para operandos numéricos
    let mut operators: Pila<&str, 25> = Pila::new(); //pila para operadores
    while let Ok(token) = tokens.dequeue() { //itera sobre cada token en la cola
        match token {
            "(" => { /* Ignored */ } //se ignora
            ")" => { //al encontrarlo, extrae un operador de operators
                let Ok(operator) = operators.pop() else {
                    return Err(String::from(ERR_MSG));
                };
                if apply_operator(operator, &mut operands).is_err() { //aplica la operación
                    return Err(String::from(ERR_MSG));
                }
            }
            "+" | "-" | "*" | "/" | "sqrt" => {
                if operators.push(token).is_err() {
                    return Err(String::from(ERR_MSG));
                }
            }
            _ => {
                let Ok(operand)  = token.parse::<f64>() else  {
                    return Err(String::from(ERR_MSG));
                };
                if operands.push(operand).is_err() { //Operadores agregados a OPERATORS
                    return Err(String::from(ERR_MSG));
                }
            }
        }
    }
    match operands.pop() {
        Ok(result) => {
            if operands.is_empty() { //extrae el último valor de operands para devolver el resultado final
                Ok(result)
            } else {
                Err(String::from(ERR_MSG)) //si hay aún elementos en operands, la expresión fue incorrecta
            }
        }
        Err(_) => { Err(String::from(ERR_MSG)) }
    }
}
//lee expresiones desde el archivo expressions.txt usando read_file::read
fn main() {
    let expressions = read_file::read("expressions.txt").unwrap();

    for e in expressions {
        println!("{}", eval(e).unwrap()); //evalúa la expresión con eval
        //y muestra el resultado
    }
}


#[cfg(test)]
mod tests {
    use super::read_file;

    #[test]
    fn test_0() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[0].clone()), Ok(28.0));
    }

    #[test]
    fn test_1() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[1].clone()), Ok(-140.0));
    }

    #[test]
    fn test_2() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[2].clone()), Ok(13.0));
    }

    #[test]
    fn test_3() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[3].clone()), Ok(104.0));
    }

    #[test]
    fn test_4() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[4].clone()), Ok(158.4));
    }

    #[test]
    fn test_5() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(
            super::eval(expressions[5].clone()),
            Err(String::from("Incorrect expression"))
        );
    }

    #[test]
    fn test_6() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[6].clone()), Ok(-2.0));
    }

    #[test]
    fn test_7() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[7].clone()), Ok(-7.0));
    }

    #[test]
    fn test_8() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(
            super::eval(expressions[8].clone()),
            Err(String::from("Incorrect expression"))
        );
    }

    #[test]
    fn test_9() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[9].clone()), Ok(3.0));
    }

    #[test]
    fn test_10() {
        let expressions = read_file::read("expressions.txt").unwrap();
        assert_eq!(super::eval(expressions[10].clone()), Ok(14.0));
    }
}
