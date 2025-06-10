//! Provides a macro to convert standard-ish Rust code into code that modifies an
//! `AllocatingBuilder`.

// #[macro_export]
// ///
// macro_rules! script {
//     ($builder:ident, {} $(, { $($output:tt)* })? $(,)?) => {
//         $($($output)*)?
//     };

//     ($builder:ident, {
//         let mut $ident:ident = $value:literal;
//         $($rest:tt)*
//     } $(, { $($output:tt)* })? $(,)?) => {
//         script!($builder, { $($rest)* }, {
//             $($($output)*)?
//             let mut $ident = $builder.into_cell($value);
//         })
//     };

//     ($builder:ident, {
//         let $ident:ident = $value:literal;
//         $($rest:tt)*
//     } $(, { $($output:tt)* })? $(,)?) => {
//         script!($builder, { $($rest)* }, {
//             $($($output)*)?
//             let $ident = $builder.into_cell($value);
//         })
//     };

//     // Assignment operators

//     ($b:ident, { $i:ident += $v:literal; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i += $v; }) };
//     ($b:ident, { $i:ident -= $v:literal; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i -= $v; }) };
//     ($b:ident, { $i:ident *= $v:literal; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i *= $v; }) };
//     ($b:ident, { $i:ident /= $v:literal; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i /= $v; }) };
//     ($b:ident, { $i:ident &= $v:literal; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i &= $v; }) };
//     ($b:ident, { $i:ident |= $v:literal; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i |= $v; }) };
//     ($b:ident, { $i:ident ^= $v:literal; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i ^= $v; }) };

//     ($b:ident, { $i:ident += $v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i += $v; }) };
//     ($b:ident, { $i:ident -= $v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i -= $v; }) };
//     ($b:ident, { $i:ident *= $v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i *= $v; }) };
//     ($b:ident, { $i:ident /= $v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i /= $v; }) };
//     ($b:ident, { $i:ident &= $v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i &= $v; }) };
//     ($b:ident, { $i:ident |= $v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i |= $v; }) };
//     ($b:ident, { $i:ident ^= $v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i ^= $v; }) };

//     ($b:ident, { $i:ident += &$v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i += &$v; }) };
//     ($b:ident, { $i:ident -= &$v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i -= &$v; }) };
//     ($b:ident, { $i:ident *= &$v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i *= &$v; }) };
//     ($b:ident, { $i:ident /= &$v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i /= &$v; }) };
//     ($b:ident, { $i:ident &= &$v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i &= &$v; }) };
//     ($b:ident, { $i:ident |= &$v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i |= &$v; }) };
//     ($b:ident, { $i:ident ^= &$v:ident; $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i ^= &$v; }) };

//     // Operators

//     ($b:ident, { $i:ident + $v:literal $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i + $v }) };
//     ($b:ident, { $i:ident - $v:literal $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i - $v }) };
//     ($b:ident, { $i:ident * $v:literal $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i * $v }) };
//     ($b:ident, { $i:ident / $v:literal $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i / $v }) };
//     ($b:ident, { $i:ident & $v:literal $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i & $v }) };
//     ($b:ident, { $i:ident | $v:literal $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i | $v }) };
//     ($b:ident, { $i:ident ^ $v:literal $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i ^ $v }) };

//     ($b:ident, { $i:ident + $v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i + $v }) };
//     ($b:ident, { $i:ident - $v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i - $v }) };
//     ($b:ident, { $i:ident * $v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i * $v }) };
//     ($b:ident, { $i:ident / $v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i / $v }) };
//     ($b:ident, { $i:ident & $v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i & $v }) };
//     ($b:ident, { $i:ident | $v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i | $v }) };
//     ($b:ident, { $i:ident ^ $v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i ^ $v }) };

//     ($b:ident, { $i:ident + &$v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i + &$v }) };
//     ($b:ident, { $i:ident - &$v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i - &$v }) };
//     ($b:ident, { $i:ident * &$v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i * &$v }) };
//     ($b:ident, { $i:ident / &$v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i / &$v }) };
//     ($b:ident, { $i:ident & &$v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i & &$v }) };
//     ($b:ident, { $i:ident | &$v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i | &$v }) };
//     ($b:ident, { $i:ident ^ &$v:ident $($r:tt)* } $(, { $($o:tt)* })? $(,)?) => { script!($b, { $($r)* }, { $($($o)*)? $i ^ &$v }) };
// }
