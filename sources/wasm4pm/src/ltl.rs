use crate::evidence::{ConstraintValue, DeclareWitnessState};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclareRule {
    Precedence(String, String),
    Response(String, String),
}

impl DeclareRule {
    /// Parse a Declare rule from a string representation.
    /// Expected formats: "Precedence(A, B)" or "Response(A, B)" (spaces allowed)
    pub fn parse(s: &str) -> Result<Self, String> {
        let s_trimmed = s.trim();
        if s_trimmed.starts_with("Precedence(") && s_trimmed.ends_with(')') {
            let content = &s_trimmed["Precedence(".len()..s_trimmed.len() - 1];
            let parts: Vec<&str> = content.split(',').collect();
            if parts.len() == 2 {
                return Ok(DeclareRule::Precedence(
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                ));
            }
            return Err(format!("Precedence rule must have exactly 2 arguments: {}", s));
        } else if s_trimmed.starts_with("Response(") && s_trimmed.ends_with(')') {
            let content = &s_trimmed["Response(".len()..s_trimmed.len() - 1];
            let parts: Vec<&str> = content.split(',').collect();
            if parts.len() == 2 {
                return Ok(DeclareRule::Response(
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                ));
            }
            return Err(format!("Response rule must have exactly 2 arguments: {}", s));
        }
        Err(format!("Unknown or malformed Declare rule: {}", s))
    }

    /// Evaluate trace satisfaction, returning ConstraintValue.
    /// If activation condition never occurs, returns PossiblySatisfied.
    pub fn evaluate(&self, trace: &[String]) -> ConstraintValue {
        match self {
            DeclareRule::Precedence(a, b) => {
                // Activation condition is B.
                let b_occurred = trace.iter().any(|x| x == b);
                if !b_occurred {
                    return ConstraintValue::PossiblySatisfied;
                }
                
                // B occurred. Ensure every occurrence of B is preceded by A.
                let mut a_occurred = false;
                for event in trace {
                    if event == a {
                        a_occurred = true;
                    }
                    if event == b && !a_occurred {
                        return ConstraintValue::Violated;
                    }
                }
                ConstraintValue::Satisfied
            }
            DeclareRule::Response(a, b) => {
                // Activation condition is A.
                let a_occurred = trace.iter().any(|x| x == a);
                if !a_occurred {
                    return ConstraintValue::PossiblySatisfied;
                }

                // A occurred. Ensure B occurs at or after the last occurrence of A.
                let last_a_idx = trace.iter().rposition(|x| x == a).unwrap();
                let b_after_last_a = trace[last_a_idx..].iter().any(|x| x == b);
                if b_after_last_a {
                    ConstraintValue::Satisfied
                } else {
                    ConstraintValue::Violated
                }
            }
        }
    }
}

/// Helper function to evaluate multiple declare rules on a trace.
pub fn evaluate_declare_rules(
    rules: &[(String, DeclareRule)],
    trace: &[String],
) -> DeclareWitnessState {
    let mut map = HashMap::new();
    for (id, rule) in rules {
        map.insert(id.clone(), rule.evaluate(trace));
    }
    DeclareWitnessState::Evaluated(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::Lattice;

    #[test]
    fn test_parsing() {
        assert_eq!(
            DeclareRule::parse("Precedence(A, B)"),
            Ok(DeclareRule::Precedence("A".to_string(), "B".to_string()))
        );
        assert_eq!(
            DeclareRule::parse("Response(A, B)"),
            Ok(DeclareRule::Response("A".to_string(), "B".to_string()))
        );
        assert_eq!(
            DeclareRule::parse("  Precedence(  A1  ,  B2  )  "),
            Ok(DeclareRule::Precedence("A1".to_string(), "B2".to_string()))
        );
        assert!(DeclareRule::parse("InvalidRule(A, B)").is_err());
        assert!(DeclareRule::parse("Precedence(A)").is_err());
    }

    #[test]
    fn test_precedence_evaluation() {
        let rule = DeclareRule::Precedence("A".to_string(), "B".to_string());

        // Vacuous satisfaction (B never occurs)
        assert_eq!(rule.evaluate(&[]), ConstraintValue::PossiblySatisfied);
        assert_eq!(
            rule.evaluate(&["A".to_string()]),
            ConstraintValue::PossiblySatisfied
        );

        // Satisfied non-vacuously (B preceded by A)
        assert_eq!(
            rule.evaluate(&["A".to_string(), "B".to_string()]),
            ConstraintValue::Satisfied
        );
        assert_eq!(
            rule.evaluate(&["A".to_string(), "B".to_string(), "B".to_string()]),
            ConstraintValue::Satisfied
        );
        assert_eq!(
            rule.evaluate(&["A".to_string(), "C".to_string(), "B".to_string()]),
            ConstraintValue::Satisfied
        );

        // Violated (B occurs before A, or A never occurs but B occurs)
        assert_eq!(
            rule.evaluate(&["B".to_string()]),
            ConstraintValue::Violated
        );
        assert_eq!(
            rule.evaluate(&["B".to_string(), "A".to_string()]),
            ConstraintValue::Violated
        );
        assert_eq!(
            rule.evaluate(&["C".to_string(), "B".to_string(), "A".to_string()]),
            ConstraintValue::Violated
        );
    }

    #[test]
    fn test_response_evaluation() {
        let rule = DeclareRule::Response("A".to_string(), "B".to_string());

        // Vacuous satisfaction (A never occurs)
        assert_eq!(rule.evaluate(&[]), ConstraintValue::PossiblySatisfied);
        assert_eq!(
            rule.evaluate(&["B".to_string()]),
            ConstraintValue::PossiblySatisfied
        );

        // Satisfied non-vacuously (A followed by B)
        assert_eq!(
            rule.evaluate(&["A".to_string(), "B".to_string()]),
            ConstraintValue::Satisfied
        );
        assert_eq!(
            rule.evaluate(&["A".to_string(), "A".to_string(), "B".to_string()]),
            ConstraintValue::Satisfied
        );
        assert_eq!(
            rule.evaluate(&["A".to_string(), "B".to_string(), "C".to_string()]),
            ConstraintValue::Satisfied
        );

        // Violated (A occurs but B never follows)
        assert_eq!(
            rule.evaluate(&["A".to_string()]),
            ConstraintValue::Violated
        );
        assert_eq!(
            rule.evaluate(&["B".to_string(), "A".to_string()]),
            ConstraintValue::Violated
        );
        assert_eq!(
            rule.evaluate(&["A".to_string(), "B".to_string(), "A".to_string()]),
            ConstraintValue::Violated
        );
    }

    #[test]
    fn test_integration_with_declare_witness_state() {
        let rule_prec = DeclareRule::Precedence("A".to_string(), "B".to_string());
        let rule_resp = DeclareRule::Response("A".to_string(), "B".to_string());

        let rules = vec![
            ("rule1".to_string(), rule_prec),
            ("rule2".to_string(), rule_resp),
        ];

        let trace1 = vec!["A".to_string(), "B".to_string()];
        let state1 = evaluate_declare_rules(&rules, &trace1);

        if let DeclareWitnessState::Evaluated(ref map) = state1 {
            assert_eq!(map.get("rule1"), Some(&ConstraintValue::Satisfied));
            assert_eq!(map.get("rule2"), Some(&ConstraintValue::Satisfied));
        } else {
            panic!("Expected Evaluated state");
        }

        let trace2 = vec!["B".to_string(), "A".to_string()];
        let state2 = evaluate_declare_rules(&rules, &trace2);

        if let DeclareWitnessState::Evaluated(ref map) = state2 {
            assert_eq!(map.get("rule1"), Some(&ConstraintValue::Violated));
            assert_eq!(map.get("rule2"), Some(&ConstraintValue::Violated));
        } else {
            panic!("Expected Evaluated state");
        }

        // Test monotonicity transition
        assert!(state1.is_monotonic_transition(&state1));
        // Transition from Bottom to any evaluated state is monotonic
        assert!(DeclareWitnessState::Bottom.is_monotonic_transition(&state1));
    }
}
