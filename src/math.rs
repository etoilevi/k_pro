use std::mem::swap;

/// Methods for Each Natural Integer Type
pub trait Natural {
    fn lcm(self, v: Self) -> Self;
    fn gcd(self, v: Self) -> Self;
    fn fact(self) -> Self;
    fn perm(self, k: Self) -> Self;
    fn comb(self, k: Self) -> Self;
}

/// Implements trait Natural
macro_rules! natural {
    ($t:ty) => {
        impl Natural for $t {
            fn lcm(self, v: $t) -> $t {
                self * v / self.gcd(v)
            }

            fn gcd(self, v: $t) -> $t {
                let (mut v1, mut v2) = (self, v);
                if v1 < v2 {
                    swap(&mut v1, &mut v2);
                }
                while v2 != 0 {
                    let t = v1;
                    v1 = v2;
                    v2 = t % v2;
                }
                return v1;
            }

            fn fact(self) -> $t {
                let mut x = 1;
                for i in 1..=self {
                    x *= i;
                }
                return x;
            }

            fn perm(self, k: $t) -> $t {
                self.fact() / (self - k).fact()
            }

            fn comb(self, k: $t) -> $t {
                self.perm(k) / k.fact()
            }
        }
    };
}

// BEGIN DECLARATIONS
natural!(u8);
natural!(u16);
natural!(u32);
natural!(u64);
natural!(u128);
natural!(usize);
// END DECLARATIONS

#[test]
fn test() {
    assert_eq!((10u64).lcm(3), 30);
    assert_eq!((10u64).lcm(25), 50);
    assert_eq!((10u64).gcd(3), 1);
    assert_eq!((10u64).gcd(25), 5);
    assert_eq!((10u64).fact(), 3628800);
    assert_eq!((10u64).perm(3), 720);
    assert_eq!((10u64).perm(5), 30240);
    assert_eq!((10u64).comb(3), 120);
    assert_eq!((10u64).comb(5), 252);
}
