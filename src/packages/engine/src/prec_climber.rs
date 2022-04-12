use crate::parser::Rule;
use pest::prec_climber::*;

lazy_static! {
    pub static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;
        PrecClimber::new(vec![
            // Primatives
              Operator::new(boolean, Left)
            | Operator::new(number, Left)
            | Operator::new(string, Left)
            | Operator::new(null, Left),

            // Math expressions
              Operator::new(add, Left)
            | Operator::new(subtract, Left),
              Operator::new(multiply, Left)
            | Operator::new(divide, Left),
              Operator::new(power, Right),

            // equality fn
              Operator::new(and, Left)
            | Operator::new(or, Left),

              Operator::new(eq, Left)
            | Operator::new(n_eq, Left),


            // Functions
              Operator::new(if_fn, Left),
        ])
    };
}
