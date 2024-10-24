#[derive(PartialEq)]
enum SymbolType {
    Number,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParentheses,
    RightParentheses,
    EndOfExpression,
}

enum Symbol {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParentheses,
    RightParentheses,
    EndOfExpression,
}

impl Symbol {
    fn symbol_type(&self) -> SymbolType {
        match self {
            Symbol::Number(_) => SymbolType::Number,
            Symbol::Plus => SymbolType::Plus,
            Symbol::Minus => SymbolType::Minus,
            Symbol::Multiply => SymbolType::Multiply,
            Symbol::Divide => SymbolType::Divide,
            Symbol::LeftParentheses => SymbolType::LeftParentheses,
            Symbol::RightParentheses => SymbolType::RightParentheses,
            Symbol::EndOfExpression => SymbolType::EndOfExpression,
        }
    }
}

/// Convert raw input into symbols
struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    /// Advance current char
    fn advance(&mut self) {
        self.current_char = self.chars.next();
    }

    /// Get integer from current char
    fn integer(&mut self) -> i32 {
        let mut result = String::new();
        while self.current_char.map_or(false, |c| c.is_ascii_digit()) {
            result.push(self.current_char.unwrap());
            self.advance();
        }
        result.parse::<i32>().unwrap()
    }

    /// Advance symbol and evaluate current symbol
    fn get_next_symbol(&mut self) -> Symbol {
        if let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                return Symbol::Number(self.integer());
            }

            let symbol = match c {
                '+' => Symbol::Plus,
                '-' => Symbol::Minus,
                '*' => Symbol::Multiply,
                '/' => Symbol::Divide,
                '(' => Symbol::LeftParentheses,
                ')' => Symbol::RightParentheses,
                _ => unreachable!("Inputs are expected to be valid."),
            };
            self.advance();
            return symbol;
        }
        Symbol::EndOfExpression
    }
}

/// Convert symbols into evaluated expression
struct Parser<'a> {
    lexer: Lexer<'a>,
    current_symbol: Symbol,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let current_symbol = lexer.get_next_symbol();
        Parser {
            lexer,
            current_symbol,
        }
    }

    /// Validate and consume current symbol
    fn consume_symbol(&mut self, symbol_type: SymbolType) {
        if self.current_symbol.symbol_type() == symbol_type {
            self.current_symbol = self.lexer.get_next_symbol();
        } else {
            unreachable!("Inputs are expected to be valid.");
        }
    }

    fn factor(&mut self) -> i32 {
        match &self.current_symbol {
            Symbol::Number(value) => {
                let val = *value;
                self.consume_symbol(SymbolType::Number);
                val
            }
            Symbol::LeftParentheses => {
                self.consume_symbol(SymbolType::LeftParentheses);
                let result = self.expr();
                self.consume_symbol(SymbolType::RightParentheses);
                result
            }
            Symbol::Plus => {
                self.consume_symbol(SymbolType::Plus);
                self.factor()
            }
            Symbol::Minus => {
                self.consume_symbol(SymbolType::Minus);
                -self.factor()
            }
            _ => unreachable!("Inputs are expected to be valid."),
        }
    }

    fn term(&mut self) -> i32 {
        let mut result = self.factor();
        loop {
            match self.current_symbol.symbol_type() {
                SymbolType::Multiply => {
                    self.consume_symbol(SymbolType::Multiply);
                    result *= self.factor();
                }
                SymbolType::Divide => {
                    self.consume_symbol(SymbolType::Divide);
                    result /= self.factor();
                }
                _ => break,
            }
        }
        result
    }

    fn expr(&mut self) -> i32 {
        let mut result = self.term();
        loop {
            match self.current_symbol.symbol_type() {
                SymbolType::Plus => {
                    self.consume_symbol(SymbolType::Plus);
                    result += self.term();
                }
                SymbolType::Minus => {
                    self.consume_symbol(SymbolType::Minus);
                    result -= self.term();
                }
                _ => break,
            }
        }
        result
    }
}

/// Add * character if multiplication is implied
fn preprocess_input(input: &str) -> String {
    let mut output = String::new();
    let mut previous: Option<char> = None;

    for c in input.chars() {
        if let Some(prev_char) = previous {
            if (prev_char.is_ascii_digit() || prev_char == ')') && c == '(' {
                output.push('*');
            } else if prev_char == ')' && c.is_ascii_digit() {
                output.push('*');
            } else if prev_char == ')' && c == '(' {
                output.push('*');
            }
        }
        output.push(c);
        previous = Some(c);
    }
    output
}

/// Evaluates a mathematical expression given as a string and returns the result.
/// # Examples
/// ```
/// assert_eq!(coderbyte_rust::calculator("3+4*2"), 11);
/// assert_eq!(coderbyte_rust::calculator("(1+3)*2"), 8);
/// ```
pub fn calculator(input: &str) -> i32 {
    let preprocessed_input = preprocess_input(input);
    let lexer = Lexer::new(&preprocessed_input);
    let mut parser = Parser::new(lexer);
    parser.expr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocessing() {
        let input = "3+4(2+1)";
        assert_eq!(preprocess_input(input), "3+4*(2+1)");
    }

    #[test]
    fn test_calculator() {
        assert_eq!(calculator("3+4*2"), 11);
        assert_eq!(calculator("(1+3)*2"), 8);
        assert_eq!(calculator("10+(2*3)-(4/2)"), 14);
        assert_eq!(calculator("7+((3+2)*(8-5))"), 22);
        assert_eq!(calculator("12/(2+4)*3"), 6);
        assert_eq!(calculator("5+((1+2)*4)-3"), 14);
        assert_eq!(calculator("100"), 100);
        assert_eq!(calculator("2*(3+5)/4"), 4);
        assert_eq!(calculator("2*3+4"), 10);
        assert_eq!(calculator("3+4(2+1)"), 15);
    }
}
