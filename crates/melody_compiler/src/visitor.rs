use crate::ast::*;

pub trait Visitor {
    fn compile(&self) -> String;
}

impl Visitor for Keyword {
    fn compile(&self) -> String {
        format!("{}", self)
    }
}

impl Visitor for Range {
    fn compile(&self) -> String {
        match self {
            Range::Exact(i) => format!("{{{}}}", i),
            Range::Numeric(r) => format!("{{{},{}}}", r.start, r.end),
            Range::Over(o) => match o.start {
                0 => format!("*"),
                1 => format!("+"),
                start => format!("{{{},}}", start),
            },
            Range::Upto(u) => format!("{{,{}}}", u.end),
        }
    }
}

impl<'a> Visitor for Expr<'a> {
    fn compile(&self) -> String {
        match self {
            Expr::Literal(l) => format!("{}", l),
            Expr::CharRange { start, end } => format!("[{}-{}]", start, end),
            Expr::DigitRange { start, end } => format!("[{}-{}]", start, end),
            Expr::Keyword(k) => k.compile(),
            Expr::Quantifier { range, expr } => {
                format!("({}{})", expr.compile(), range.compile())
            }
            Expr::Capture {
                capture_name,
                capture_exprs,
            } => match capture_name {
                Some(capture_name) => format!("(?<{}>{})", capture_name, capture_exprs.compile(),),
                None => format!("({})", capture_exprs.compile()),
            },
            Expr::Match {
                match_name,
                match_exprs,
            } => match match_name {
                Some(match_name) => format!("(?:<{}>{})", match_name, match_exprs.compile()),
                None => format!("(?:{})", match_exprs.compile()),
            },
        }
    }
}

impl<'a> Visitor for MultipleExprs<'a> {
    fn compile(&self) -> String {
        self.0.iter().fold(String::new(), |mut result, expr| {
            result.push_str(&expr.compile());
            result
        })
    }
}
