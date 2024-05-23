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
    Prod(Box<Self>, Box<Self>),
    Rec(Box<Self>),
    Pow(Box<Self>, Box<Self>),
    // Trig fns:
    Sin(Angle),
    Cos(Angle),
}

#[derive(Clone)]
pub enum Angle {
    Pi(Box<Val>),
    // // Numeric ops:
    Sum(Box<Self>, Box<Self>),
    // Trig fns:
    ASin(Box<Val>),
    ACos(Box<Val>),
}

impl Val {
    pub fn add(self, a: &Val) -> Self {
        // If either value is a literal zero, just return the other one.
        if self.is_zero() {
            a.clone()
        } else if a.is_zero() {
            self
        } else {
            match (&self, &a) {
                // Otherwise try to push down the operation.
                (Self::Int(x), Self::Int(y)) => Self::Int(x + y),
                // If that doesn't work, box the sum into a new enum.
                _ => Self::Sum(Box::new(self), Box::new(a.clone())),
            }
        }
    }

    pub fn sub(self, a: &Val) -> Self {
        self.add(&a.clone().neg())
    }

    pub fn neg(self) -> Self {
        match self {
            // Try to push down the operation.
            Self::Int(x) => Self::Int(-x),
            _ => self.mul(&Self::from(-1)),
        }
    }

    pub fn mul(self, a: &Val) -> Self {
        // If either value is a literal zero, just return zero.
        // Or, if either value is a literal one, just return the other one.
        if self.is_zero() || a.is_zero() {
            0.into()
        } else if self.is_one() {
            a.clone()
        } else if a.is_one() {
            self
        } else {
            match (&self, &a) {
                // Otherwise try to push down the operation.
                (Self::Int(x), Self::Int(y)) => Self::Int(x * y),
                // If that doesn't work, bodx the product int a new enum.
                _ => Self::Prod(Box::new(self), Box::new(a.clone())),
            }
        }
    }

    pub fn div(self, a: &Val) -> Self {
        if a.is_zero() {
            panic!("?/0")
        } else if self.is_zero() {
            0.into()
        } else if a.is_one() {
            self
        } else {
            self.mul(&a.clone().rec())
        }
    }

    pub fn rec(self) -> Self {
        if self.is_zero() {
            panic!("1/0")
        } else if self.is_one() {
            1.into()
        } else {
            Self::Rec(Box::new(self))
        }
    }

    pub fn pow(self, a: &Val) -> Self {
        if self.is_zero() && a.is_zero() {
            panic!("0^0")
        } else if self.is_zero() {
            0.into()
        } else if a.is_zero() {
            1.into()
        } else {
            Self::Pow(Box::new(self), Box::new(a.clone()))
        }
    }

    pub fn sqrt(self) -> Self {
        if self.is_zero() {
            0.into()
        } else if self.is_one() {
            1.into()
        } else {
            self.pow(&Val::from(1).div(&2.into()))
        }
    }

    pub fn pi(self) -> Angle {
        Angle::Pi(Box::new(self))
    }

    pub fn asin(self) -> Angle {
        match self {
            Self::Sin(x) => x,
            _ => Angle::ASin(Box::new(self)),
        }
    }

    pub fn acos(self) -> Angle {
        match self {
            Self::Cos(x) => x,
            _ => Angle::ACos(Box::new(self)),
        }
    }

    fn is_zero(&self) -> bool {
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

    /// Adds another angle to this one.
    pub fn add(self, a: &Self) -> Self {
        if self.is_zero() {
            a.clone()
        } else if a.is_zero() {
            self
        } else {
            match (&self, &a) {
                (Self::Pi(ref x), Self::Pi(ref y)) => Self::Pi(Box::new(x.clone().add(y))),
                _ => Self::Sum(Box::new(self), Box::new(a.clone())),
            }
        }
    }

    pub fn mul(self, a: &Val) -> Self {
        match self {
            Self::Pi(x) => Self::Pi(Box::new(x.mul(a))),
            _ => todo!(),
        }
    }

    pub fn div(self, a: &Val) -> Self {
        match self {
            Self::Pi(x) => Self::Pi(Box::new(x.div(a))),
            _ => todo!(),
        }
    }

    pub fn sin(self) -> Val {
        if self.is_zero() {
            0.into()
        } else {
            Val::Sin(self)
        }
    }

    pub fn cos(self) -> Val {
        if self.is_zero() {
            1.into()
        } else {
            Val::Cos(self)
        }
    }

    /// Checks if this angle is the literal zero angle.
    /// Currently doesn't return true for e.g. 2n*pi for all integer n.
    fn is_zero(&self) -> bool {
        match self {
            Self::Pi(x) => x.is_zero(),
            _ => false,
        }
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
            Self::Prod(a, b) => a.to_i64().and_then(|x| b.to_i64().map(|y| x * y)),
            Self::Rec(a) => match &**a {
                Self::Rec(x) => x.clone().to_i64(),
                _ => a.to_i64().and_then(|x| if x == 1 { Some(1) } else { None }),
            },
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
            Self::Prod(a, b) => a.to_u64().and_then(|x| b.to_u64().map(|y| x * y)),
            Self::Rec(a) => match &**a {
                Self::Rec(x) => x.clone().to_u64(),
                _ => a.to_u64().and_then(|x| if x == 1 { Some(1) } else { None }),
            },
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
            Self::Prod(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x * y)),
            Self::Rec(a) => a.to_f64().and_then(|x| Some(1.0 / x)),
            Self::Pow(a, b) => a.to_f64().and_then(|x| b.to_f64().map(|y| x.powf(y))),
            Self::Sin(a) => a.to_f64().and_then(|x| Some(x.sin())),
            Self::Cos(a) => a.to_f64().and_then(|x| Some(x.cos())),
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
            Self::ASin(a) => a.to_f64().and_then(|x| Some(x.asin())),
            Self::ACos(a) => a.to_f64().and_then(|x| Some(x.acos())),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(a) => write!(f, "{}", a),
            Self::Sum(a, b) => write!(f, "({} + {})", a, b),
            Self::Prod(a, b) => write!(f, "({} * {})", a, b),
            Self::Rec(a) => write!(f, "(1 / {})", a),
            Self::Pow(a, b) => write!(f, "({}^{})", a, b),
            Self::Sin(a) => write!(f, "sin({})", a),
            Self::Cos(a) => write!(f, "cos({})", a),
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
                    write!(f, "{} * pi", a)
                }
            }
            Self::Sum(a, b) => write!(f, "({} + {})", a, b),
            Self::ASin(a) => write!(f, "asin({})", a),
            Self::ACos(a) => write!(f, "acos({})", a),
        }
    }
}

pub fn sqrt(a: Val) -> Val {
    a.sqrt()
}
