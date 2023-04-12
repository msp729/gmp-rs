use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Shl, ShlAssign, Sub, SubAssign,
};
use std::fmt::Display;
use std::ops::{Neg, Shr, ShrAssign};

use crate::{Limb, RandState};

pub type BitCount = u64;

macro_rules! todo_ {
    ($mp:ty | $tr:ty [ $func:ident ]) => {
        impl $tr for &$mp {
            type Output = Self;
            fn $func(self, _: Self) -> Self {
                todo!()
            }
        }
    };
    ($mp:ty | $tr:ty [ = $func:ident ]) => {
        impl $tr for &$mp {
            fn $func(&mut self, _: Self) {
                todo!()
            }
        }
    };
    (fun $name:ident ($($args:ty),*) -> $out:ty) => {
        pub fn $name($(_:$args),*) -> $out {
            todo!()
        }
    };
    (ifun $name:ident ($($args:ty),*) -> $out:ty) => {
        pub fn $name(&self, $(_:$args),*) -> $out {
            todo!()
        }
    };
    (mfun $name:ident ($($args:ty),*) -> $out:ty) => {
        pub fn $name(&mut self, $(_:$args),*) -> $out {
            todo!()
        }
    };
    ($src:ty => $dst:ty) => {
        impl Into<$dst> for $src {
            fn into(self) -> $dst {
                todo!()
            }
        }
    };
    ($src:ty, from $dst:ty) => {
        impl From<$src> for $dst {
            fn from(_:$src) -> Self {
                todo!()
            }
        }
    };
}

pub enum Endianness {
    Big,
    Little,
    Native,
}

pub struct MPZ;
impl MPZ {
    todo_!(fun from_str_radix(&str, u8) -> Self);
    todo_!(ifun to_str_radix(u8) -> &str);
    todo_!(ifun to_str_raw() -> &str);
    todo_!(fun from_str_raw(&str) -> Result<MPZ, &str>);
    todo_!(ifun to_d_2exp() -> (f64, i64));
    todo_!(fun gcd(&MPZ, &MPZ) -> MPZ);
    todo_!(fun lcm(&MPZ, &MPZ) -> MPZ);
    todo_!(fun urandomb(RandState, BitCount) -> MPZ);
    todo_!(fun urandomm(RandState, BitCount) -> MPZ);
    todo_!(fun rrandomb(RandState, BitCount) -> MPZ);
    pub fn import(_: Endianness, _: usize, _: &[usize]) -> MPZ {
        todo!()
    }
    pub fn export(&self, _: Endianness, _: usize) -> &[usize] {
        todo!()
    }
    todo_!(mfun modify() -> &mut [Limb]);
    todo_!(mfun finish(usize) -> ());
    todo_!(ifun getlimb(u64) -> Option<Limb>);
    todo_!(ifun size() -> u64);
    todo_!(ifun iseven() -> bool);
    todo_!(ifun isodd() -> bool);
    todo_!(ifun size_in_base(u8) -> u64);
    todo_!(fun jacobi(&MPZ, &MPZ) -> MPZ);
    todo_!(fun legendre(&MPZ, &MPZ) -> MPZ);
    todo_!(fun kronecker(&MPZ, &MPZ) -> MPZ);
    // TODO: mpz_remove
    todo_!(fun factorial(&u128) -> MPZ);
    todo_!(fun bifactorial(&u128) -> MPZ);
    todo_!(fun multifactorial(&u128, &u128) -> MPZ);
    todo_!(fun primorial(&u128) -> MPZ);
    todo_!(fun fibonacci(&u128) -> MPZ);
    todo_!(fun fibonacci2(&u128) -> (MPZ, MPZ));
    todo_!(fun lucas(&u128) -> MPZ);
    todo_!(fun lucas2(&u128) -> (MPZ, MPZ));
    todo_!(fun binomial(&MPZ, &u128) -> MPZ);
    todo_!(ifun modinverse(&MPZ) -> MPZ);
    todo_!(ifun abs() -> MPZ);
    todo_!(ifun powm(&MPZ, &MPZ) -> MPZ);
    todo_!(ifun pow(&MPZ) -> MPZ);
    todo_!(ifun root(u128) -> Result<MPZ, MPZ>);
    todo_!(ifun rootrem(u128) -> (MPZ, MPZ));
    todo_!(ifun probably_prime(isize) -> bool);
    todo_!(ifun sgn() -> i8);
    todo_!(ifun popcount() -> BitCount);
    todo_!(fun hamdist(&MPZ, &MPZ) -> BitCount);
    todo_!(ifun scan0(BitCount) -> BitCount);
    todo_!(ifun scan1(BitCount) -> BitCount);
    todo_!(mfun setbit(BitCount) -> ());
    todo_!(mfun clrbit(BitCount) -> ());
    todo_!(mfun combit(BitCount) -> ());
    todo_!(ifun tstbit(BitCount) -> bool);
}
todo_!(MPZ => u128);
todo_!(MPZ => u64);
todo_!(MPZ => u32);
todo_!(MPZ => i128);
todo_!(MPZ => i64);
todo_!(MPZ => i32);
todo_!(MPZ => f64);
impl Display for MPZ {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
todo_!(MPZ | Add[add]);
todo_!(MPZ | Sub[sub]);
todo_!(MPZ | Mul[mul]);
todo_!(MPZ | Div[div]);
impl Neg for &MPZ {
    type Output = MPZ;

    fn neg(self) -> MPZ {
        todo!()
    }
}
todo_!(MPZ|AddAssign[=add_assign]);
todo_!(MPZ|SubAssign[=sub_assign]);
todo_!(MPZ|MulAssign[=mul_assign]);
todo_!(MPZ|DivAssign[=div_assign]);

todo_!(MPZ | Shl[shl]);
todo_!(MPZ | Shr[shr]);
todo_!(MPZ|ShlAssign[=shl_assign]);
todo_!(MPZ|ShrAssign[=shr_assign]);

todo_!(MPZ | BitAnd[bitand]);
todo_!(MPZ | BitXor[bitxor]);
todo_!(MPZ | BitOr[bitor]);
todo_!(MPZ | BitAndAssign[=bitand_assign]);
todo_!(MPZ | BitXorAssign[=bitxor_assign]);
todo_!(MPZ | BitOrAssign[=bitor_assign]);

impl Not for MPZ {
    type Output = Self;

    fn not(self) -> Self::Output {
        todo!()
    }
}

impl Ord for MPZ {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        todo!()
    }
}
impl Eq for MPZ {}
impl PartialOrd for MPZ {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}
impl PartialEq for MPZ {
    fn eq(&self, _: &Self) -> bool {
        todo!()
    }
}
