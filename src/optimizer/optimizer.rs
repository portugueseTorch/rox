use crate::parser::{ast::AstNode, statements::Stmt};

pub struct Optimizer;

impl Optimizer {
    pub fn optimize(ast: Vec<Stmt>) -> Vec<Stmt> {
        let initial_node_count = Optimizer::count_nodes(&ast);
        println!("Optimization started at {} nodes", initial_node_count);

        // let optimized_stmts = vec![];
        // for stmt in ast {
        //     optimized_stmts.push(stmt.optimize());
        // }
        unimplemented!();
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
        parser.parse()
    }

    #[test]
    fn count_nodes_1() {
        let ast = scan_and_parse("2 + 3");
        let node_count = Optimizer::count_nodes(&ast);

        assert_eq!(node_count, 3);
    }

    #[test]
    fn count_nodes_2() {
        let ast = scan_and_parse("2 + 3 * 42");
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
        let ast = scan_and_parse("fun myFunc(a, b) { var myVar = a; return a + 42;}");
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
}
