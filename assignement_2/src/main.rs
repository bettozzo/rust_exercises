#[allow(dead_code)]
enum Operation{
    Add,
    Sub,
    Mul,
    Div,
}
enum Expression{
    Number(i32),
    Operation(Box<Expression>, Box<Expression>, Operation), 
}



fn main(){
    //exp = 5*(5+2)
    let exp =   Expression::Operation(Box::new(Expression::Number(5)),
                            Box::new(Expression::Operation(Box::new(Expression::Number(5)), Box::new(Expression::Number(2)), Operation::Add)),
                            Operation::Mul);
    match evalute_expression(exp){
        Ok(res) => println!("{}", res),
        Err(e) => println!("scemo {}", e),
    }
}

fn evalute_expression(exp: Expression) -> Result<i32, String>{
    return match exp{
        Expression::Number(num) => return Ok(num),
        Expression::Operation(exp_l, exp_r, op)
        => match op {
             Operation::Mul =>  Ok(evalute_expression(*exp_l)?*evalute_expression(*exp_r)?),
             Operation::Div =>  {
                 let den = evalute_expression(*exp_r)?;
                 if den == 0{
                    return Err(String::from("DIVISIONE PER 00000000"));
                 }
                 Ok(evalute_expression(*exp_l)?/den)
             },
            Operation::Add =>  Ok(evalute_expression(*exp_l)?+evalute_expression(*exp_r)?),
            Operation::Sub =>  Ok(evalute_expression(*exp_l)?-evalute_expression(*exp_r)?),
        }
    };
}