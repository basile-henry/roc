#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate indoc;

extern crate bumpalo;
extern crate inkwell;
extern crate libc;
extern crate roc_gen;

#[macro_use]
mod helpers;

#[cfg(test)]
mod gen_records {
    #[test]
    fn basic_record() {
        assert_evals_to!(
            indoc!(
                r#"
                    { y: 17, x: 15, z: 19 }.x
                "#
            ),
            15,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    { x: 15, y: 17, z: 19 }.y
                "#
            ),
            17,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    { x: 15, y: 17, z: 19 }.z
                "#
            ),
            19,
            i64
        );
    }

    #[test]
    fn f64_record() {
        assert_evals_to!(
            indoc!(
                r#"
                   rec = { y: 17.2, x: 15.1, z: 19.3 }

                   rec.x
                "#
            ),
            15.1,
            f64
        );

        assert_evals_to!(
            indoc!(
                r#"
                   rec = { y: 17.2, x: 15.1, z: 19.3 }

                   rec.y
                "#
            ),
            17.2,
            f64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    rec = { y: 17.2, x: 15.1, z: 19.3 }

                    rec.z
                "#
            ),
            19.3,
            f64
        );
    }

    #[test]
    fn fn_record() {
        assert_evals_to!(
            indoc!(
                r#"
                    getRec = \x -> { y: 17, x, z: 19 }

                    (getRec 15).x
                "#
            ),
            15,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    rec = { x: 15, y: 17, z: 19 }

                    rec.y
                "#
            ),
            17,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    rec = { x: 15, y: 17, z: 19 }

                    rec.z
                "#
            ),
            19,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    rec = { x: 15, y: 17, z: 19 }

                    rec.z + rec.x
                "#
            ),
            34,
            i64
        );
    }

    #[test]
    fn def_record() {
        assert_evals_to!(
            indoc!(
                r#"
                    rec = { y: 17, x: 15, z: 19 }

                    rec.x
                "#
            ),
            15,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    rec = { x: 15, y: 17, z: 19 }

                    rec.y
                "#
            ),
            17,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    rec = { x: 15, y: 17, z: 19 }

                    rec.z
                "#
            ),
            19,
            i64
        );
    }

    #[test]
    fn when_on_record() {
        assert_evals_to!(
            indoc!(
                r#"
                when { x: 0x2 } is
                    { x } -> x + 3
                "#
            ),
            5,
            i64
        );
    }

    #[test]
    fn when_on_record_with_guard_pattern() {
        assert_evals_to!(
            indoc!(
                r#"
                when { x: 0x2, y: 3.14 } is
                    { x: var } -> var + 3
                "#
            ),
            5,
            i64
        );
    }

    #[test]
    fn let_with_record_pattern() {
        assert_evals_to!(
            indoc!(
                r#"
                { x } = { x: 0x2, y: 3.14 }

                x
                "#
            ),
            2,
            i64
        );
    }

    #[test]
    fn record_guard_pattern() {
        assert_evals_to!(
            indoc!(
                r#"
                when { x: 0x2, y: 3.14 } is
                    { x: 0x4 } -> 5
                    { x } -> x + 3
                "#
            ),
            5,
            i64
        );
    }

    #[test]
    fn twice_record_access() {
        assert_evals_to!(
            indoc!(
                r#"
                x =  {a: 0x2, b: 0x3 }

                x.a + x.b
                "#
            ),
            5,
            i64
        );
    }
    #[test]
    fn empty_record() {
        assert_evals_to!(
            indoc!(
                r#"
                v = {}

                1
                "#
            ),
            1,
            i64
        );
    }
    #[test]
    fn i64_record2_literal() {
        assert_evals_to!(
            indoc!(
                r#"
                   { x: 3, y: 5 }
                "#
            ),
            (3, 5),
            (i64, i64)
        );
    }

    // #[test]
    // fn i64_record3_literal() {
    //     assert_evals_to!(
    //         indoc!(
    //             r#"
    //                { x: 3, y: 5, z: 17 }
    //             "#
    //         ),
    //         (3, 5, 17),
    //         (i64, i64, i64)
    //     );
    // }

    #[test]
    fn f64_record2_literal() {
        assert_evals_to!(
            indoc!(
                r#"
                   { x: 3.1, y: 5.1 }
                "#
            ),
            (3.1, 5.1),
            (f64, f64)
        );
    }

    // #[test]
    // fn f64_record3_literal() {
    //     assert_evals_to!(
    //         indoc!(
    //             r#"
    //                { x: 3.1, y: 5.1, z: 17.1 }
    //             "#
    //         ),
    //         (3.1, 5.1, 17.1),
    //         (f64, f64, f64)
    //     );
    // }

    // #[test]
    // fn bool_record4_literal() {
    //     assert_evals_to!(
    //         indoc!(
    //             r#"
    //                record : { a : Bool, b : Bool, c : Bool, d : Bool }
    //                record = { a: True, b: True, c : True, d : Bool }

    //                record
    //             "#
    //         ),
    //         (true, false, false, true),
    //         (bool, bool, bool, bool)
    //     );
    // }

    #[test]
    fn i64_record1_literal() {
        assert_evals_to!(
            indoc!(
                r#"
                   { a: 3 }
                "#
            ),
            3,
            i64
        );
    }

    // #[test]
    // fn i64_record9_literal() {
    //     assert_evals_to!(
    //         indoc!(
    //             r#"
    //                { a: 3, b: 5, c: 17, d: 1, e: 9, f: 12, g: 13, h: 14, i: 15 }
    //             "#
    //         ),
    //         (3, 5, 17, 1, 9, 12, 13, 14, 15),
    //         (i64, i64, i64, i64, i64, i64, i64, i64, i64)
    //     );
    // }

    // #[test]
    // fn f64_record3_literal() {
    //     assert_evals_to!(
    //         indoc!(
    //             r#"
    //                { x: 3.1, y: 5.1, z: 17.1 }
    //             "#
    //         ),
    //         (3.1, 5.1, 17.1),
    //         (f64, f64, f64)
    //     );
    // }

    #[test]
    fn bool_literal() {
        assert_evals_to!(
            indoc!(
                r#"
                x : Bool
                x = True

                x
                "#
            ),
            true,
            bool
        );
    }

    #[test]
    fn return_record() {
        assert_evals_to!(
            indoc!(
                r#"
                x = 4
                y = 3

                { x, y }
                "#
            ),
            (4, 3),
            (i64, i64)
        );
    }

    #[test]
    fn optional_field_when_use_default() {
        assert_evals_to!(
            indoc!(
                r#"
                f = \r ->
                    when r is
                        { x: Blue, y ? 3 } -> y
                        { x: Red, y ? 5 } -> y

                a = f { x: Blue, y: 7 } 
                b = f { x: Blue }
                c = f { x: Red, y: 11 } 
                d = f { x: Red }

                a * b * c * d
                "#
            ),
            3 * 5 * 7 * 11,
            i64
        );
    }

    #[test]
    fn optional_field_when_no_use_default() {
        assert_evals_to!(
            indoc!(
                r#"
                f = \r ->
                    { x ? 10, y } = r
                    x + y

                f { x: 4, y: 9 }
                "#
            ),
            13,
            i64
        );
    }

    #[test]
    fn optional_field_let_use_default() {
        assert_evals_to!(
            indoc!(
                r#"
                f = \r ->
                    { x ? 10, y } = r
                    x + y

                f { y: 9 }
                "#
            ),
            19,
            i64
        );
    }

    #[test]
    fn optional_field_let_no_use_default() {
        assert_evals_to!(
            indoc!(
                r#"
                f = \r ->
                    { x ? 10, y } = r
                    x + y

                f { x: 4, y: 9 }
                "#
            ),
            13,
            i64
        );
    }

    #[test]
    fn optional_field_function_use_default() {
        assert_evals_to!(
            indoc!(
                r#"
                f = \{ x ? 10, y } -> x + y


                f { y: 9 }
                "#
            ),
            19,
            i64
        );
    }

    #[test]
    fn optional_field_function_no_use_default() {
        assert_evals_to!(
            indoc!(
                r#"
                f = \{ x ? 10, y } -> x + y


                f { x: 4, y: 9 }
                "#
            ),
            13,
            i64
        );
    }
}
