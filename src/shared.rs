#[macro_export]
macro_rules! wrapping_add {
    ($v:expr, $a:expr, $b:expr, $c:expr) => (
        $v = $a.wrapping_add(
            $b.wrapping_add(
                $c
            )
        )
    )
}
