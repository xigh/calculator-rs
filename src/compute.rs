use std::f64::consts::E;
use crate::parser::Expr;

pub fn execute(expr: Expr) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(n),
        Expr::BinaryOp(left, op, right) => {
            let left_val = execute(*left)?;
            let right_val = execute(*right)?;
            match op {
                '+' => Ok(left_val + right_val),
                '-' => Ok(left_val - right_val),
                '*' => Ok(left_val * right_val),
                '/' => {
                    if right_val == 0.0 {
                        Err("division by zero".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                },
                _ => Err(format!("unknown operator: {}", op)),
            }
        },
        Expr::Function(name, arg) => {
            let arg_val = execute(*arg)?;
            match name.as_str() {
                "sin" => Ok(arg_val.sin()),
                "cos" => Ok(arg_val.cos()),
                "tan" => Ok(arg_val.tan()),
                "exp" => Ok(arg_val.exp()),
                "log" => Ok(arg_val.log(E)),
                "sqrt" => Ok(arg_val.sqrt()),
                "log2" => Ok(arg_val.log2()),
                "log10" => Ok(arg_val.log10()),
                _ => Err(format!("unknown function: {}", name)),
            }
        },
        Expr::UnaryOp(op, expr) => {
            let val = execute(*expr)?;
            match op {
                '+' => Ok(val),
                '-' => Ok(-val),
                _ => Err(format!("unknown postfix operator: {}", op)),
            }
        },
    }
}
