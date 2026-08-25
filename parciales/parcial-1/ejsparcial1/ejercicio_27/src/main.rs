use cola::Cola;
use pila::Pila;
use tokens::next_token;


mod tokens;
mod cola;
mod pila;
mod read_file;


const ERR_MSG: &str = "Incorrect expression";


fn apply_operator(operator: &str, operands: &mut Pila<f64, 25>) -> Result<(), String> {
    let Ok(right_operand) = operands.pop() else {
        return Err(String::from(ERR_MSG));
    };
    if operator == "sqrt" {
        if operands.push(right_operand.sqrt()).is_err() {
            return Err(String::from(ERR_MSG));
        }
    } else {
        let Ok(left_operand) = operands.pop() else {
            return Err(String::from(ERR_MSG));
        };
        let result: f64 = match operator {
            "+" => left_operand + right_operand,
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
    Ok(())
}

fn eval(e: String) -> Result<f64, String> {
    let mut tokens: Cola<&str, 50> = Cola::new();
    let mut start = 0;
    while let Some((begin, end)) = next_token(&e, start) {
        if tokens.enqueue(&e[begin..=end]).is_err() {
            return Err(String::from(ERR_MSG));
        }
        start = end + 1;
    }


    let mut operands: Pila<f64, 25> = Pila::new();
    let mut operators: Pila<&str, 25> = Pila::new();
    while let Ok(token) = tokens.dequeue() {
        match token {
            "(" => { /* Ignored */ }
            ")" => {
                let Ok(operator) = operators.pop() else {
                    return Err(String::from(ERR_MSG));
                };
                if apply_operator(operator, &mut operands).is_err() {
                    return Err(String::from(ERR_MSG));
                }
            }
            "+" | "-" | "*" | "/" | "sqrt" => { // Operator
                if operators.push(token).is_err() {
                    return Err(String::from(ERR_MSG));
                }
            }
            _ => {
                let Ok(operand)  = token.parse::<f64>() else  {
                    return Err(String::from(ERR_MSG));
                };
                if operands.push(operand).is_err() {
                    return Err(String::from(ERR_MSG));
                }
            }
        }
    }
    match operands.pop() {
        Ok(result) => {
            if operands.is_empty() {
                Ok(result)
            } else {
                Err(String::from(ERR_MSG))
            }
        }
        Err(_) => { Err(String::from(ERR_MSG)) }
    }
}

fn main() {
    let expressions = read_file::read("expressions.txt").unwrap();

    for e in expressions {
        println!("{}", eval(e).unwrap());
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
