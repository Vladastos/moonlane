/// Test suite for the type inference system.
/// Tests are organized by phase/component matching the task breakdown.

#[cfg(test)]
mod phase_1_type_variables {
    use yolang::typeinference::{TypeVar, TypeVarGenerator};

    #[test]
    fn test_type_var_creation() {
        let var1 = TypeVar(0);
        let var2 = TypeVar(1);

        assert_eq!(var1.0, 0);
        assert_eq!(var2.0, 1);
        assert_ne!(var1, var2);
    }

    #[test]
    fn test_type_var_display() {
        let var = TypeVar(42);
        assert_eq!(format!("{}", var), "?t42");
    }

    #[test]
    fn test_type_var_generator_fresh() {
        let mut gen = TypeVarGenerator::new();

        let v1 = gen.fresh();
        let v2 = gen.fresh();
        let v3 = gen.fresh();

        assert_eq!(v1.0, 0);
        assert_eq!(v2.0, 1);
        assert_eq!(v3.0, 2);
        assert_ne!(v1, v2);
        assert_ne!(v2, v3);
    }

    #[test]
    fn test_type_var_generator_counter() {
        let mut gen = TypeVarGenerator::new();
        assert_eq!(gen.counter(), 0);

        gen.fresh();
        assert_eq!(gen.counter(), 1);

        gen.fresh();
        gen.fresh();
        assert_eq!(gen.counter(), 3);
    }

    #[test]
    fn test_type_var_ordering() {
        let v0 = TypeVar(0);
        let v1 = TypeVar(1);
        let v5 = TypeVar(5);

        assert!(v0 < v1);
        assert!(v1 < v5);
        assert!(v0 < v5);
    }

    #[test]
    fn test_type_var_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(TypeVar(0));
        set.insert(TypeVar(1));
        set.insert(TypeVar(0));  // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&TypeVar(0)));
        assert!(set.contains(&TypeVar(1)));
        assert!(!set.contains(&TypeVar(2)));
    }
}

#[cfg(test)]
mod phase_2_infer_types {
    use yolang::typeinference::{InferType, TypeVar};
    use yolang::types::Type;

    #[test]
    fn test_concrete_variants() {
        assert_eq!(InferType::int(), InferType::Concrete(Type::Int));
        assert_eq!(InferType::float(), InferType::Concrete(Type::Float));
        assert_eq!(InferType::bool(), InferType::Concrete(Type::Bool));
        assert_eq!(InferType::str(), InferType::Concrete(Type::Str));
        assert_eq!(InferType::unit(), InferType::Concrete(Type::Unit));
    }

    #[test]
    fn test_var_constructor() {
        let v = TypeVar(3);
        assert_eq!(InferType::var(v), InferType::Var(TypeVar(3)));
    }

    #[test]
    fn test_display_concrete() {
        assert_eq!(format!("{}", InferType::int()), "Int");
        assert_eq!(format!("{}", InferType::float()), "Float");
        assert_eq!(format!("{}", InferType::bool()), "Bool");
        assert_eq!(format!("{}", InferType::str()), "String");
        assert_eq!(format!("{}", InferType::unit()), "()");
    }

    #[test]
    fn test_display_var() {
        assert_eq!(format!("{}", InferType::var(TypeVar(0))), "?t0");
        assert_eq!(format!("{}", InferType::var(TypeVar(7))), "?t7");
    }

    #[test]
    fn test_display_fun() {
        let ty = InferType::Fun(
            vec![InferType::int(), InferType::bool()],
            Box::new(InferType::str()),
        );
        assert_eq!(format!("{}", ty), "fun(Int, Bool) -> String");
    }

    #[test]
    fn test_display_fun_no_params() {
        let ty = InferType::Fun(vec![], Box::new(InferType::unit()));
        assert_eq!(format!("{}", ty), "fun() -> ()");
    }

    #[test]
    fn test_display_tuple() {
        let ty = InferType::Tuple(vec![InferType::int(), InferType::bool()]);
        assert_eq!(format!("{}", ty), "(Int, Bool)");
    }

    #[test]
    fn test_display_array() {
        let ty = InferType::Array(Box::new(InferType::int()));
        assert_eq!(format!("{}", ty), "Int[]");
    }

    #[test]
    fn test_display_named_no_args() {
        let ty = InferType::Named("Foo".to_string(), vec![]);
        assert_eq!(format!("{}", ty), "Foo");
    }

    #[test]
    fn test_display_named_with_args() {
        let ty = InferType::Named("Map".to_string(), vec![InferType::str(), InferType::int()]);
        assert_eq!(format!("{}", ty), "Map<String, Int>");
    }

    #[test]
    fn test_display_with_type_vars() {
        let ty = InferType::Fun(
            vec![InferType::var(TypeVar(0))],
            Box::new(InferType::var(TypeVar(0))),
        );
        assert_eq!(format!("{}", ty), "fun(?t0) -> ?t0");
    }

    #[test]
    fn test_nested_types() {
        // Array of functions: fun(Int) -> Bool []
        let ty = InferType::Array(Box::new(InferType::Fun(
            vec![InferType::int()],
            Box::new(InferType::bool()),
        )));
        assert_eq!(format!("{}", ty), "fun(Int) -> Bool[]");
    }

    #[test]
    fn test_equality() {
        let a = InferType::Fun(
            vec![InferType::var(TypeVar(0))],
            Box::new(InferType::var(TypeVar(1))),
        );
        let b = InferType::Fun(
            vec![InferType::var(TypeVar(0))],
            Box::new(InferType::var(TypeVar(1))),
        );
        let c = InferType::Fun(
            vec![InferType::var(TypeVar(0))],
            Box::new(InferType::var(TypeVar(2))),
        );
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}

// Placeholder for Phase 3 tests
#[cfg(test)]
mod phase_3_unification {
    // TODO: Add unification tests here
}

// Placeholder for Phase 4 tests
#[cfg(test)]
mod phase_4_substitution {
    // TODO: Add substitution tests here
}

// Placeholder for Phase 5 tests
#[cfg(test)]
mod phase_5_constraints {
    // TODO: Add constraint tests here
}

// Placeholder for Phase 6 tests
#[cfg(test)]
mod phase_6_type_schemes {
    // TODO: Add type scheme tests here
}

// Placeholder for Phase 7 tests
#[cfg(test)]
mod phase_7_infer_context {
    // TODO: Add inference context tests here
}
