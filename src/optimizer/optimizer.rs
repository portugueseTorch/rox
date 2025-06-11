use crate::parser::{ast::AstNode, statements::Stmt};

pub struct Optimizer;

impl Optimizer {
    pub fn optimize(ast: Vec<Stmt>) -> Vec<Stmt> {
        let initial_node_count = Optimizer::count_nodes(&ast);
        println!("Optimization started at {} nodes", initial_node_count);

        let mut optimized_stmts = vec![];
        for stmt in ast {
            optimized_stmts.push(stmt.optimize());
        }

        let final_node_count = Optimizer::count_nodes(&optimized_stmts);
        println!("Optimization ended at {} nodes", final_node_count);
        optimized_stmts
    }

    pub fn count_nodes(ast: &Vec<Stmt>) -> usize {
        ast.iter().map(|m| m.count_nodes()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{parser::Parser, statements::Stmt},
        scanner::scanner::Scanner,
    };

    use super::Optimizer;

    fn scan_and_parse<'a>(src: &'a str) -> Vec<Stmt<'a>> {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan().unwrap();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        if parser.has_errors() {
            parser.log_errors();
        }
        ast
    }

    #[test]
    fn count_nodes_1() {
        let ast = scan_and_parse("2 + 3;");
        let node_count = Optimizer::count_nodes(&ast);

        assert_eq!(node_count, 3);
    }

    #[test]
    fn count_nodes_2() {
        let ast = scan_and_parse("2 + 3 * 42;");
        let node_count = Optimizer::count_nodes(&ast);

        assert_eq!(node_count, 5);
    }

    #[test]
    fn count_nodes_3() {
        let ast = scan_and_parse(
            "if (true) {
                42;
            }",
        );
        let node_count = Optimizer::count_nodes(&ast);

        assert_eq!(node_count, 2);
    }

    #[test]
    fn count_nodes_4() {
        let ast = scan_and_parse(
            "fun myFunc(a, b) {
                var myVar = a;
                return a + 42;
            }",
        );
        let node_count = Optimizer::count_nodes(&ast);

        assert_eq!(node_count, 4);
    }

    #[test]
    fn count_nodes_5() {
        let ast = scan_and_parse(
            "
            for (var i = 1; i < 10; i = i + 1) {
                42 + a;
            }
            ",
        );
        let node_count = Optimizer::count_nodes(&ast);

        assert_eq!(node_count, 11);
    }

    #[test]
    fn optimize_1() {
        let ast = scan_and_parse("var myVar = 42 + 1 * 1 + 4 * 2;");
        ast.iter().for_each(|stmt| println!("Before: {}", stmt));
        let optimized = Optimizer::optimize(ast);
        assert_eq!(Optimizer::count_nodes(&optimized), 1);
    }

    #[test]
    fn optimize_2() {
        let ast = scan_and_parse("var myVar = 42 + 4 * 2 - 40 / (2 * 5);");
        ast.iter().for_each(|stmt| println!("Before: {}", stmt));
        let optimized = Optimizer::optimize(ast);
        assert_eq!(Optimizer::count_nodes(&optimized), 1);
    }

    #[test]
    fn optimize_3() {
        let ast =
            scan_and_parse("obj.myFunc(2 * 5, 42 + 3 * 6, test.method(10 + 3 * 10 + 20 / 2));");
        ast.iter().for_each(|stmt| println!("Before: {}", stmt));
        let optimized = Optimizer::optimize(ast);
        optimized
            .iter()
            .for_each(|stmt| println!("After: {}", stmt));
    }
}
