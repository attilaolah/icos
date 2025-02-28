extern crate num_bigint;
extern crate num_traits;

use std::f64::consts::PI;
use std::fmt;

use num_bigint::BigInt as Int;
use num_traits::{One, ToPrimitive, Zero};

#[derive(Clone)]
pub enum Val {
    Int(Int),
    // Numeric ops:
    Sum(Box<Self>, Box<Self>),
    Dif(Box<Self>, Box<Self>),
    Prd(Box<Self>, Box<Self>),
    Rat(Box<Self>, Box<Self>),
    Pow(Box<Self>, Box<Self>),
    Sqrt(Box<Self>),
    // Trig fns:
    Sin(Angle),
    Cos(Angle),
    Tan(Angle),
    // Parameter:
    Param(usize),
}

#[derive(Clone)]
pub enum Angle {
    Pi(Box<Val>),
    // Numeric ops:
    Sum(Box<Self>, Box<Self>),
    Dif(Box<Self>, Box<Self>),
    Prd(Box<Self>, Box<Val>),
    Rat(Box<Self>, Box<Val>),
    // Trig fns:
    ASin(Box<Val>),
    ACos(Box<Val>),
    ATan(Box<Val>),
}

impl Val {
    pub fn param(t: usize) -> Self {
        Self::Param(t)
    }

    pub fn add(&self, a: &Val) -> Self {
        // If either value is a literal zero, just return the other one.
        if self.is_zero() {
            a.clone()
        } else if a.is_zero() {
            self.clone()
        } else {
            match (&self, &a) {
                // Otherwise try to push down the operation.
                (Self::Int(x), Self::Int(y)) => Self::Int(x + y),
                // If that doesn't work, box the sum into a new enum.
                _ => Self::Sum(Box::new(self.clone()), Box::new(a.clone())),
            }
        }
    }

    pub fn sub(&self, a: &Val) -> Self {
        self.add(&a.neg())
    }

    pub fn neg(&self) -> Self {
        match self {
            // Try to push down the operation.
            Self::Int(x) => Self::Int(-x),
            Self::Dif(x, y) => Self::Dif(y.clone(), x.clone()),
            Self::Prd(x, y) => Self::Prd(Box::new(x.neg()), y.clone()),
            Self::Rat(x, y) => Self::Rat(Box::new(x.neg()), y.clone()),
            _ => self.mul(&Self::from(-1)),
        }
    }

    pub fn mul(&self, a: &Val) -> Self {
        // If either value is a literal zero, just return zero.
        // Or, if either value is a literal one, just return the other one.
        if self.is_zero() || a.is_zero() {
            0.into()
        } else if self.is_one() {
            a.clone()
        } else if a.is_one() {
            self.clone()
        } else {
            match (&self, &a) {
                // Otherwise try to push down the operation.
                (Self::Int(x), Self::Int(y)) => Self::Int(x * y),
                // If that doesn't work, bodx the product int a new enum.
                _ => Self::Prd(Box::new(self.clone()), Box::new(a.clone())),
            }
        }
    }

    pub fn div(&self, a: &Val) -> Self {
        if a.is_zero() {
            panic!("?/0")
        } else if self.is_zero() {
            0.into()
        } else if a.is_one() {
            self.clone()
        } else {
            Self::Rat(Box::new(self.clone()), Box::new(a.clone()))
        }
    }

    pub fn rec(&self) -> Self {
        if self.is_zero() {
            panic!("1/0")
        } else if self.is_one() {
            1.into()
        } else {
            match &self {
                Self::Rat(a, b) => Self::Rat(b.clone(), a.clone()),
                _ => Self::Rat(Box::from(Val::from(1)), Box::from(self.clone())),
            }
        }
    }

    pub fn pow(&self, a: &Val) -> Self {
        if self.is_zero() && a.is_zero() {
            panic!("0^0")
        } else if self.is_zero() {
            0.into()
        } else if a.is_zero() {
            1.into()
        } else {
            Self::Pow(Box::new(self.clone()), Box::new(a.clone()))
        }
    }

    pub fn sqrt(&self) -> Self {
        if self.is_zero() {
            0.into()
        } else if self.is_one() {
            1.into()
        } else {
            Self::Sqrt(Box::new(self.clone()))
        }
    }

    pub fn pi(&self) -> Angle {
        Angle::Pi(Box::new(self.clone()))
    }

    pub fn asin(&self) -> Angle {
        match self {
            Self::Sin(x) => x.clone(),
            _ => Angle::ASin(Box::new(self.clone())),
        }
    }

    pub fn acos(&self) -> Angle {
        match self {
            Self::Cos(x) => x.clone(),
            _ => Angle::ACos(Box::new(self.clone())),
        }
    }

    pub fn atan(&self) -> Angle {
        match self {
            Self::Tan(x) => x.clone(),
            _ => Angle::ATan(Box::new(self.clone())),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Int(x) => x.is_zero(),
            _ => false,
        }
    }

    fn is_one(&self) -> bool {
        match self {
            Self::Int(x) => x.is_one(),
            _ => false,
        }
    }

    // Shortcuts:
    pub fn iadd(&self, a: i64) -> Self {
        self.add(&a.into())
    }
    pub fn isub(&self, a: i64) -> Self {
        self.sub(&a.into())
    }
    pub fn imul(&self, a: i64) -> Self {
        self.mul(&a.into())
    }
    pub fn idiv(&self, a: i64) -> Self {
        self.div(&a.into())
    }
    pub fn ipow(&self, a: i64) -> Self {
        self.pow(&a.into())
    }
}

impl Angle {
    /// Zero angle.
    pub fn zero() -> Self {
        Self::Pi(Box::new(0.into()))
    }

    /// A full turn.
    pub fn turn() -> Self {
        Self::Pi(Box::new(2.into()))
    }

    /// A partial turn.
    pub fn part(a: i64) -> Self {
        Self::turn().idiv(a)
    }

    /// Adds another angle to this one.
    pub fn add(&self, a: &Self) -> Self {
        if self.is_zero() {
            a.clone()
        } else if a.is_zero() {
            self.clone()
        } else {
            match (&self, &a) {
                (Self::Pi(ref x), Self::Pi(ref y)) => Self::Pi(Box::new(x.clone().add(y))),
                _ => Self::Sum(Box::new(self.clone()), Box::new(a.clone())),
            }
        }
    }

    /// Substitutes another angle from this one.
    pub fn sub(&self, a: &Self) -> Self {
        if self.is_zero() {
            a.clone().neg()
        } else if a.is_zero() {
            self.clone()
        } else {
            match (&self, &a) {
                (Self::Pi(ref x), Self::Pi(ref y)) => Self::Pi(Box::new(x.clone().sub(y))),
                _ => Self::Dif(Box::new(self.clone()), Box::new(a.clone())),
            }
        }
    }

    pub fn neg(&self) -> Self {
        self.imul(-1)
    }

    pub fn mul(&self, a: &Val) -> Self {
        match self {
            Self::Pi(x) => Self::Pi(Box::new(x.mul(a))),
            _ => Self::Prd(Box::new(self.clone()), Box::new(a.clone())),
        }
    }

    pub fn div(&self, a: &Val) -> Self {
        match self {
            Self::Pi(x) => Self::Pi(Box::new(x.div(a))),
            _ => Self::Rat(Box::new(self.clone()), Box::new(a.clone())),
        }
    }

    pub fn sin(&self) -> Val {
        if self.is_zero() {
            0.into()
        } else {
            Val::Sin(self.clone())
        }
    }

    pub fn cos(&self) -> Val {
        if self.is_zero() {
            1.into()
        } else {
            Val::Cos(self.clone())
        }
    }

    pub fn tan(&self) -> Val {
        if self.is_zero() {
            0.into()
        } else {
            Val::Tan(self.clone())
        }
    }

    /// Checks if this angle is the literal zero angle.
    /// Currently doesn't return true for e.g. 2n*pi for all integer n.
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Pi(x) => x.is_zero(),
            _ => false,
        }
    }

    // Shortcuts:
    pub fn imul(&self, a: i64) -> Self {
        self.mul(&a.into())
    }
    pub fn idiv(&self, a: i64) -> Self {
        self.div(&a.into())
    }
}

impl From<i64> for Val {
    fn from(item: i64) -> Self {
        Val::Int(item.into())
    }
}

impl ToPrimitive for Val {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::Int(a) => a.to_i64(),
            Self::Sum(a, b) => a.to_i64().and_then(|x| b.to_i64().map(|y| x + y)),
            Self::Dif(a, b) => a.to_i64().and_then(|x| b.to_i64().map(|y| x - y)),
            Self::Prd(a, b) => a.to_i64().and_then(|x| b.to_i64().map(|y| x * y)),
            Self::Pow(a, b) => a.to_i64().and_then(|x| {
                b.to_u64().and_then(|y| match y.try_into() {
                    Ok(y) => Some(x.pow(y)),
                    Err(_) => None,
                })
            }),
            _ => None,
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Self::Int(a) => a.to_u64(),
            Self::Sum(a, b) => a.to_u64().and_then(|x| b.to_u64().map(|y| x + y)),
            Self::Dif(a, b) => a.to_u64().and_then(|x| b.to_u64().map(|y| x - y)),
            Self::Prd(a, b) => a.to_u64().and_then(|x| b.to_u64().map(|y| x * y)),
            Self::Pow(a, b) => a.to_u64().and_then(|x| {
                b.to_u64().and_then(|y| match y.try_into() {
                    Ok(y) => Some(x.pow(y)),
                    Err(_) => None,
                })
            }),
            _ => None,
        }
    }

    /// Converts the value to a float.
    /// No efforts are made for numeric stability; use only for debugging.
    fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Int(a) => a.to_f64(),
            Self::Sum(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x + y)),
            Self::Dif(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x - y)),
            Self::Prd(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x * y)),
            Self::Rat(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x / y)),
            Self::Pow(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x.powf(y))),
            Self::Sqrt(a) => a.to_f64().and_then(|x| Some(x.sqrt())),
            Self::Sin(a) => a.to_f64().and_then(|x| Some(x.sin())),
            Self::Cos(a) => a.to_f64().and_then(|x| Some(x.cos())),
            Self::Tan(a) => a.to_f64().and_then(|x| Some(x.tan())),
            Self::Param(_) => None,
        }
    }
}

impl ToPrimitive for Angle {
    fn to_i64(&self) -> Option<i64> {
        self.to_f64().and_then(|x| Some(x as i64))
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_i64().and_then(|x| Some(x as u64))
    }

    /// Converts the value to a float.
    /// No efforts are made for numeric stability; use only for debugging.
    fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Pi(a) => a.to_f64().and_then(|x| Some(PI * x)),
            Self::Sum(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x + y)),
            Self::Dif(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x - y)),
            Self::Prd(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x * y)),
            Self::Rat(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x / y)),
            Self::ASin(a) => a.to_f64().and_then(|x| Some(x.asin())),
            Self::ACos(a) => a.to_f64().and_then(|x| Some(x.acos())),
            Self::ATan(a) => a.to_f64().and_then(|x| Some(x.atan())),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(a) => write!(f, "{}", a),
            Self::Sum(a, b) => write!(f, "({}+{})", a, b),
            Self::Dif(a, b) => write!(f, "({}-{})", a, b),
            Self::Prd(a, b) => write!(f, "({}*{})", a, b),
            Self::Rat(a, b) => write!(f, "({}/{})", a, b),
            Self::Pow(a, b) => write!(f, "pow({},{})", a, b),
            Self::Sqrt(a) => write!(f, "sqrt({})", a),
            Self::Sin(a) => write!(f, "sin({})", a),
            Self::Cos(a) => write!(f, "cos({})", a),
            Self::Tan(a) => write!(f, "tan({})", a),
            Self::Param(t) => {
                if *t == 0 {
                    write!(f, "t")
                } else {
                    write!(f, "t_{}", t)
                }
            }
        }
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pi(a) => {
                if a.is_zero() {
                    write!(f, "0")
                } else {
                    write!(f, "{}*PI", a)
                }
            }
            Self::Sum(a, b) => write!(f, "({}+{})", a, b),
            Self::Dif(a, b) => write!(f, "({}-{})", a, b),
            Self::Prd(a, b) => write!(f, "({}*{})", a, b),
            Self::Rat(a, b) => write!(f, "({}/{})", a, b),
            Self::ASin(a) => write!(f, "asin({})", a),
            Self::ACos(a) => write!(f, "acos({})", a),
            Self::ATan(a) => write!(f, "atan({})", a),
        }
    }
}
