#[derive(Debug, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

pub fn eval(expr: Node) -> Result<f64, Box<dyn std::error::Error>> {
    match expr {
        Node::Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Node::Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Node::Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Node::Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Node::Power(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
        Node::Negative(expr) => Ok(-(eval(*expr)?)),
        Node::Number(num) => Ok(num),
    }
}
