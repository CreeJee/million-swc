#[cfg(test)]
mod tests {
    use million_swc::utils::is_use_client::is_use_client;
    use swc_core::common::*;
    use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput};
    use swc_core::ecma::parser::{Syntax, TsConfig};

    #[test]
    fn syntax_use_client() {
        let tsx_code = r#"
        'use client'
        import React from 'react';

        const App = () => {
            return <div>Hello, world!</div>;
        };

        export default App;
        "#;

        // Create lexer and parser
        let lexer: Lexer<'_> = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                no_early_errors: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::new(tsx_code, BytePos(0), BytePos(tsx_code.len() as u32)),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        // Parse JavaScript code
        let program = parser.parse_program().expect("Failed to parse program");

        // Print the AST
        assert_eq!(is_use_client(&program), true);
    }

    #[test]
    fn syntax_no_use_client() {
        let tsx_code = r#"
        import React from 'react';

        const App = () => {
            return <div>Hello, world!</div>;
        };

        export default App;
        "#;

        // Create lexer and parser
        let lexer: Lexer<'_> = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                no_early_errors: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::new(tsx_code, BytePos(0), BytePos(tsx_code.len() as u32)),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        // Parse JavaScript code
        let program = parser.parse_program().expect("Failed to parse program");

        // Print the AST
        assert_eq!(is_use_client(&program), false);
    }
}
