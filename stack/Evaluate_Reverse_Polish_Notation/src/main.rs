impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(tokens.len());
        for token in tokens.iter() {
            if let Ok(token) = token.parse::<i32>() {
                stack.push(token);
            } else {
                let (rhs, lhs) = (stack.pop().unwrap(), stack.pop().unwrap());
                match token.as_str() {
                    "+" => stack.push(lhs + rhs),
                    "-" => stack.push(lhs - rhs),
                    "/" => stack.push(lhs / rhs),
                    "*" => stack.push(lhs * rhs),
                    _ => {}
                }
            }
        }
        stack[0]
    }
}