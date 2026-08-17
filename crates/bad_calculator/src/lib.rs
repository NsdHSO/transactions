pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn evaluat_expression(arg: &str) -> Result<String, String> {
    let parts: Vec<&str> = arg.split('+').map(str::trim).collect();
    if parts.len() != 2 {
        return Err(format!("unsupported expression: {arg}"));
    }
    let left: u64 = parts[0]
        .parse()
        .map_err(|_| format!("invalid number: {}", parts[0]))?;
    let right: u64 = parts[1]
        .parse()
        .map_err(|_| format!("invalid number: {}", parts[1]))?;
    Ok(add(left, right).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn evaluates_add_expression() {
        assert_eq!(evaluat_expression("2 + 3"), Ok("5".to_string()));
    }
}
