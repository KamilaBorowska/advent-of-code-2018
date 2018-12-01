#[macro_export]
macro_rules! test {
    (__internal $ident:tt . $part:tt) => {};
    (
        __internal $ident:tt . $part:tt
        $name:ident: $input:expr => $output:expr,
        $($tests:tt)*
    ) => {
        #[test]
        fn $name() {
            assert_eq!(($ident.$part)($input).unwrap(), $output);
        }
        super::test!(__internal $ident.$part $($tests)*);
    };
    (
        __internal $ident:tt . $part:tt
        fn $name:ident() $b:block
        $($tests:tt)*
    ) => {
        #[test] fn $name() $b
        super::test!(__internal $ident.$part $($tests)*);
    };
    (
        $ident:tt . $part:tt,
        $($tests:tt)*
    ) => {
        mod $part {
            use crate::lines;
            use super::super::$ident;
            super::test!(__internal $ident.$part $($tests)*);
        }
    };
}

#[macro_export]
macro_rules! lines {
    (__internal $out:tt) => {
        concat!$out
    };
    (__internal ($($out:tt)*) - $input:tt $($rest:tt)*) => {
        lines!(__internal ($($out)* concat!('-', $input), '\n',) $($rest)*)
    };
    (__internal ($($out:tt)*) $input:tt $($rest:tt)*) => {
        lines!(__internal ($($out)* $input, '\n',) $($rest)*)
    };
    ($($t:tt)*) => {
        lines!(__internal () $($t)*)
    };
}