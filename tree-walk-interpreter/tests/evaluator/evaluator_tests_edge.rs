/// Edge-case tests for the evaluator.
/// Covers arithmetic boundaries, runtime errors, short-circuit evaluation,
/// scoping, nested signal propagation, and all pattern kinds.
///
/// Assertion pattern: `let _ok = match (actual == expected) { true => 0, };`
/// If the condition is false no arm matches → runtime "no arm matched" panic → test fails.

#[cfg(test)]
mod tests {
    use yoloscript::{evaluator, parser, typechecker};

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn run(src: &str) {
        let ast = parser::parse(src, "test").expect("parse error");
        let prog = typechecker::check(ast).expect("typecheck error");
        evaluator::evaluate(prog).expect("runtime error");
    }

    fn run_err(src: &str) -> String {
        let ast = parser::parse(src, "test").expect("parse error");
        let prog = typechecker::check(ast).expect("typecheck error");
        evaluator::evaluate(prog).expect_err("expected runtime error").to_string()
    }

    // ── Integer arithmetic edge cases ─────────────────────────────────────────

    #[test]
    fn int_div_truncates_toward_zero_positive() {
        run(r#"
            fun main() {
                let _ok = match (7 / 2 == 3) { true => 0, };
            }
        "#);
    }

    #[test]
    fn int_div_truncates_toward_zero_negative() {
        // Rust (and Yoloscript) integer division truncates toward zero, not floor.
        run(r#"
            fun main() {
                let _ok = match (-7 / 2 == -3) { true => 0, };
            }
        "#);
    }

    #[test]
    fn int_rem_sign_follows_dividend() {
        run(r#"
            fun main() {
                let _ok1 = match (10 % 3 == 1)   { true => 0, };
                let _ok2 = match (-10 % 3 == -1)  { true => 0, };
                let _ok3 = match (10 % -3 == 1)   { true => 0, };
            }
        "#);
    }

    #[test]
    fn int_div_by_zero_is_runtime_error() {
        let err = run_err(r#"
            fun main() { let _x = 1 / 0; }
        "#);
        assert!(err.contains("division by zero"), "got: {err}");
    }

    #[test]
    fn int_rem_by_zero_is_runtime_error() {
        let err = run_err(r#"
            fun main() { let _x = 1 % 0; }
        "#);
        assert!(err.contains("remainder by zero"), "got: {err}");
    }

    #[test]
    fn float_arithmetic() {
        run(r#"
            fun main() {
                let sum  = 1.5 + 2.5;
                let diff = 5.0 - 1.5;
                let prod = 2.0 * 3.0;
                let quot = 7.0 / 2.0;
                let _ok1 = match (sum  == 4.0) { true => 0, };
                let _ok2 = match (diff == 3.5) { true => 0, };
                let _ok3 = match (prod == 6.0) { true => 0, };
                let _ok4 = match (quot == 3.5) { true => 0, };
            }
        "#);
    }

    #[test]
    fn float_comparison() {
        run(r#"
            fun main() {
                let _ok1 = match (1.0 < 2.0)  { true => 0, };
                let _ok2 = match (2.0 > 1.0)  { true => 0, };
                let _ok3 = match (1.0 == 1.0) { true => 0, };
                let _ok4 = match (1.0 != 2.0) { true => 0, };
                let _ok5 = match (1.0 <= 1.0) { true => 0, };
                let _ok6 = match (2.0 >= 1.0) { true => 0, };
            }
        "#);
    }

    // ── Short-circuit logical operators ───────────────────────────────────────

    #[test]
    fn and_short_circuits_on_false_lhs() {
        // RHS (1/0) must not be evaluated.
        run(r#"
            fun main() {
                let x = false && (1 / 0 == 0);
                let _ok = match x { false => 0, };
            }
        "#);
    }

    #[test]
    fn or_short_circuits_on_true_lhs() {
        run(r#"
            fun main() {
                let x = true || (1 / 0 == 0);
                let _ok = match x { true => 0, };
            }
        "#);
    }

    #[test]
    fn and_evaluates_rhs_when_lhs_true() {
        // When LHS is true the RHS IS evaluated → division by zero panics.
        let err = run_err(r#"
            fun main() { let _x = true && (1 / 0 == 0); }
        "#);
        assert!(err.contains("division by zero"), "got: {err}");
    }

    #[test]
    fn or_evaluates_rhs_when_lhs_false() {
        let err = run_err(r#"
            fun main() { let _x = false || (1 / 0 == 0); }
        "#);
        assert!(err.contains("division by zero"), "got: {err}");
    }

    #[test]
    fn logical_operator_results() {
        run(r#"
            fun main() {
                let _ok1 = match (true  && true)  { true  => 0, };
                let _ok2 = match (true  && false) { false => 0, };
                let _ok3 = match (false && true)  { false => 0, };
                let _ok4 = match (true  || false) { true  => 0, };
                let _ok5 = match (false || false) { false => 0, };
                let _ok6 = match (false || true)  { true  => 0, };
            }
        "#);
    }

    // ── Unary operators ───────────────────────────────────────────────────────

    #[test]
    fn double_negation() {
        run(r#"
            fun main() {
                let x = 5;
                let _ok = match (-(-x) == x) { true => 0, };
            }
        "#);
    }

    #[test]
    fn double_not() {
        run(r#"
            fun main() {
                let _ok1 = match (!!true  == true)  { true => 0, };
                let _ok2 = match (!!false == false) { true => 0, };
            }
        "#);
    }

    // ── Array indexing edge cases ─────────────────────────────────────────────

    #[test]
    fn array_last_valid_index() {
        run(r#"
            fun main() {
                let arr = [10, 20, 30];
                let last = arr[2];
                let _ok = match (last == 30) { true => 0, };
            }
        "#);
    }

    #[test]
    fn array_negative_index_is_runtime_error() {
        let err = run_err(r#"
            fun main() {
                let arr = [1, 2, 3];
                let _x = arr[-1];
            }
        "#);
        assert!(err.contains("out of bounds"), "got: {err}");
    }

    #[test]
    fn array_index_at_len_is_runtime_error() {
        let err = run_err(r#"
            fun main() {
                let arr = [1, 2, 3];
                let _x = arr[3];
            }
        "#);
        assert!(err.contains("out of bounds"), "got: {err}");
    }

    // ── Tuple access edge cases ───────────────────────────────────────────────

    #[test]
    fn tuple_access_nested() {
        run(r#"
            fun main() {
                let t = (1, (2, 3));
                let inner = t.1;
                let b = inner.0;
                let c = inner.1;
                let _ok1 = match (b == 2) { true => 0, };
                let _ok2 = match (c == 3) { true => 0, };
            }
        "#);
    }

    #[test]
    fn tuple_access_out_of_bounds_is_typecheck_error() {
        // The typechecker catches tuple OOB statically (E0003) — it never reaches runtime.
        let ast = parser::parse(r#"
            fun main() {
                let t = (1, 2);
                let _x = t.5;
            }
        "#, "test").expect("parse error");
        let err = typechecker::check(ast).expect_err("expected typecheck error");
        assert!(err.to_string().contains("out of bounds"), "got: {err}");
    }

    // ── Range iteration edge cases ────────────────────────────────────────────

    #[test]
    fn range_empty_produces_no_iterations() {
        run(r#"
            fun main() {
                mut count = 0;
                for (let _i in 0..0) {
                    count += 1;
                }
                let _ok = match (count == 0) { true => 0, };
            }
        "#);
    }

    #[test]
    fn range_reverse_produces_no_iterations() {
        run(r#"
            fun main() {
                mut count = 0;
                for (let _i in 5..1) {
                    count += 1;
                }
                let _ok = match (count == 0) { true => 0, };
            }
        "#);
    }

    #[test]
    fn range_single_element() {
        run(r#"
            fun main() {
                mut sum = 0;
                for (let i in 7..8) {
                    sum += i;
                }
                let _ok = match (sum == 7) { true => 0, };
            }
        "#);
    }

    #[test]
    fn range_inclusive_single_element() {
        run(r#"
            fun main() {
                mut sum = 0;
                for (let i in 5..=5) {
                    sum += i;
                }
                let _ok = match (sum == 5) { true => 0, };
            }
        "#);
    }

    #[test]
    fn range_inclusive_empty_when_start_exceeds_end() {
        run(r#"
            fun main() {
                mut count = 0;
                for (let _i in 5..=4) {
                    count += 1;
                }
                let _ok = match (count == 0) { true => 0, };
            }
        "#);
    }

    // ── Scoping ───────────────────────────────────────────────────────────────

    #[test]
    fn if_inner_binding_shadows_outer_correctly() {
        run(r#"
            fun main() {
                let x = 1;
                let inner_x = if (true) { let x = 99; x } else { 0 };
                let _ok1 = match (inner_x == 99) { true => 0, };
                let _ok2 = match (x == 1)        { true => 0, };
            }
        "#);
    }

    #[test]
    fn for_in_binding_does_not_leak_into_outer_scope() {
        run(r#"
            fun main() {
                let x = 42;
                mut sum = 0;
                for (let x in [1, 2, 3]) {
                    sum += x;
                }
                let _ok1 = match (sum == 6)  { true => 0, };
                let _ok2 = match (x == 42)   { true => 0, };
            }
        "#);
    }

    #[test]
    fn loop_body_scope_is_fresh_each_iteration() {
        // mut j is re-declared each outer iteration; changes inside inner loop
        // are not visible in the next outer iteration's j.
        run(r#"
            fun main() {
                mut total = 0;
                mut i = 0;
                loop {
                    i += 1;
                    if (i > 3) { break; }
                    mut j = 0;
                    loop {
                        j += 1;
                        if (j >= 2) { break; }
                    }
                    total += j;
                }
                let _ok = match (total == 6) { true => 0, };
            }
        "#);
    }

    // ── Nested signal propagation ─────────────────────────────────────────────

    #[test]
    fn break_only_breaks_innermost_loop() {
        run(r#"
            fun main() {
                mut outer_ran = 0;
                loop {
                    while (true) {
                        break;
                    }
                    outer_ran = 1;
                    break;
                }
                let _ok = match (outer_ran == 1) { true => 0, };
            }
        "#);
    }

    #[test]
    fn for_continue_still_executes_step() {
        // continue in a C-style for must still run the step expression.
        run(r#"
            fun main() {
                mut count = 0;
                for (mut i = 0; i < 5; i += 1) {
                    if (i == 2) { continue; }
                    count += 1;
                }
                let _ok = match (count == 4) { true => 0, };
            }
        "#);
    }

    #[test]
    fn return_propagates_through_nested_loops() {
        // return from inside two nested loops exits main entirely.
        // The unreachable 1/0 would cause an error if reached.
        run(r#"
            fun main() {
                loop {
                    loop {
                        return;
                    }
                    let _never = 1 / 0;
                }
            }
        "#);
    }

    #[test]
    fn break_in_for_in_stops_early() {
        run(r#"
            fun main() {
                mut sum = 0;
                for (let x in [1, 2, 3, 4, 5]) {
                    if (x == 3) { break; }
                    sum += x;
                }
                let _ok = match (sum == 3) { true => 0, };
            }
        "#);
    }

    #[test]
    fn continue_in_for_in_skips_iteration() {
        run(r#"
            fun main() {
                mut sum = 0;
                for (let x in [1, 2, 3, 4, 5]) {
                    if (x == 3) { continue; }
                    sum += x;
                }
                let _ok = match (sum == 12) { true => 0, };
            }
        "#);
    }

    #[test]
    fn while_with_false_initial_condition_never_executes_body() {
        run(r#"
            fun main() {
                mut ran = 0;
                while (false) {
                    ran = 1;
                }
                let _ok = match (ran == 0) { true => 0, };
            }
        "#);
    }

    // ── Match patterns — all variants ─────────────────────────────────────────

    #[test]
    fn match_nope_pattern() {
        run(r#"
            fun main() {
                let x: Perhaps<Int> = nope;
                let result = match x {
                    nope => 1,
                    _    => 0,
                };
                let _ok = match (result == 1) { true => 0, };
            }
        "#);
    }

    #[test]
    fn match_string_literal() {
        run(r#"
            fun main() {
                let s = "hello";
                let result = match s {
                    "world" => 1,
                    "hello" => 2,
                    _       => 3,
                };
                let _ok = match (result == 2) { true => 0, };
            }
        "#);
    }

    #[test]
    fn match_bool_literal() {
        run(r#"
            fun main() {
                let _ok1 = match true  { true  => 0, };
                let _ok2 = match false { false => 0, };
            }
        "#);
    }

    #[test]
    fn match_float_literal() {
        run(r#"
            fun main() {
                let f = 3.14;
                let result = match f {
                    1.0  => 1,
                    3.14 => 2,
                    _    => 3,
                };
                let _ok = match (result == 2) { true => 0, };
            }
        "#);
    }

    #[test]
    fn match_tuple_pattern_binds_elements() {
        run(r#"
            fun main() {
                let pair: (Int, Int) = (3, 4);
                let result = match pair {
                    (a, b) => a + b,
                };
                let _ok = match (result == 7) { true => 0, };
            }
        "#);
    }

    #[test]
    fn match_nested_tuple_pattern() {
        run(r#"
            fun main() {
                let nested: (Int, (Int, Int)) = (1, (2, 3));
                let result = match nested {
                    (a, (b, c)) => a + b + c,
                };
                let _ok = match (result == 6) { true => 0, };
            }
        "#);
    }

    #[test]
    fn match_first_arm_wins() {
        // Both binding patterns would match; only the first should fire.
        run(r#"
            fun main() {
                let result = match 42 {
                    first  => 1,
                    second => 2,
                };
                let _ok = match (result == 1) { true => 0, };
            }
        "#);
    }

    #[test]
    fn match_guard_uses_bound_variable() {
        run(r#"
            fun main() {
                let result = match 10 {
                    n if n > 5 => n * 2,
                    _          => 0,
                };
                let _ok = match (result == 20) { true => 0, };
            }
        "#);
    }

    #[test]
    fn match_guard_failure_falls_through_to_next_arm() {
        run(r#"
            fun main() {
                let result = match 5 {
                    n if n == 1 => 10,
                    n if n == 5 => 50,
                    _           => 0,
                };
                let _ok = match (result == 50) { true => 0, };
            }
        "#);
    }

    #[test]
    fn match_enum_with_all_variants() {
        run(r#"
            enum Dir { North, South, East, West }
            fun main() {
                let _n = match Dir::North { Dir::North => 0, Dir::South => 1, Dir::East => 2, Dir::West => 3, };
                let _s = match Dir::South { Dir::North => 0, Dir::South => 1, Dir::East => 2, Dir::West => 3, };
                let _e = match Dir::East  { Dir::North => 0, Dir::South => 1, Dir::East => 2, Dir::West => 3, };
                let _w = match Dir::West  { Dir::North => 0, Dir::South => 1, Dir::East => 2, Dir::West => 3, };
                let _ok1 = match (_n == 0) { true => 0, };
                let _ok2 = match (_s == 1) { true => 0, };
                let _ok3 = match (_e == 2) { true => 0, };
                let _ok4 = match (_w == 3) { true => 0, };
            }
        "#);
    }

    // ── Compound assignment operators ─────────────────────────────────────────

    #[test]
    fn assign_all_compound_int_operators() {
        run(r#"
            fun main() {
                mut n = 20;
                n -= 5;
                let _ok1 = match (n == 15) { true => 0, };
                n *= 2;
                let _ok2 = match (n == 30) { true => 0, };
                n /= 3;
                let _ok3 = match (n == 10) { true => 0, };
                n %= 3;
                let _ok4 = match (n == 1)  { true => 0, };
            }
        "#);
    }

    #[test]
    fn assign_compound_float_operators() {
        run(r#"
            fun main() {
                mut f = 10.0;
                f += 5.0;
                let _ok1 = match (f == 15.0) { true => 0, };
                f -= 3.0;
                let _ok2 = match (f == 12.0) { true => 0, };
                f *= 2.0;
                let _ok3 = match (f == 24.0) { true => 0, };
                f /= 4.0;
                let _ok4 = match (f == 6.0)  { true => 0, };
            }
        "#);
    }

    #[test]
    fn assign_plain_equals_overwrites() {
        run(r#"
            fun main() {
                mut x = 1;
                x = 99;
                let _ok = match (x == 99) { true => 0, };
            }
        "#);
    }

    // ── Cast edge cases ───────────────────────────────────────────────────────

    #[test]
    fn cast_large_int_to_float() {
        run(r#"
            fun main() {
                let big = 1000000;
                let f = big as Float;
                let _ok = match (f == 1000000.0) { true => 0, };
            }
        "#);
    }

    #[test]
    fn cast_negative_int_to_float() {
        run(r#"
            fun main() {
                let n = -7;
                let f = n as Float;
                let _ok = match (f == -7.0) { true => 0, };
            }
        "#);
    }

    // ── Miscellaneous ─────────────────────────────────────────────────────────

    #[test]
    fn string_equality() {
        run(r#"
            fun main() {
                let _ok1 = match ("abc" == "abc") { true  => 0, };
                let _ok2 = match ("abc" != "xyz") { true  => 0, };
                let _ok3 = match ("abc" == "xyz") { false => 0, };
            }
        "#);
    }

    #[test]
    fn bool_equality() {
        run(r#"
            fun main() {
                let _ok1 = match (true  == true)  { true  => 0, };
                let _ok2 = match (false == false) { true  => 0, };
                let _ok3 = match (true  != false) { true  => 0, };
                let _ok4 = match (true  == false) { false => 0, };
            }
        "#);
    }

    #[test]
    fn top_level_let_bindings_visible_in_main() {
        // Top-level let bindings (evaluated before main) are in scope when main runs.
        run(r#"
            let global = 42;
            fun main() {
                let _ok = match (global == 42) { true => 0, };
            }
        "#);
    }

    #[test]
    fn nested_if_expressions() {
        run(r#"
            fun main() {
                let x = 5;
                let result = if (x > 0) {
                    if (x > 10) { 2 } else { 1 }
                } else {
                    0
                };
                let _ok = match (result == 1) { true => 0, };
            }
        "#);
    }

    #[test]
    fn loop_accumulates_across_iterations() {
        // Verifies that assignments inside a loop body are visible in subsequent iterations.
        run(r#"
            fun main() {
                mut acc = 0;
                mut i = 1;
                loop {
                    if (i > 5) { break; }
                    acc += i;
                    i += 1;
                }
                let _ok = match (acc == 15) { true => 0, };
            }
        "#);
    }
}
