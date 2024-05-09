extern crate num_bigint;
extern crate num_traits;

use std::f64::consts::PI;
use std::fmt;

use num_bigint::BigInt as Int;
use num_traits::ToPrimitive;

#[derive(Clone)]
pub enum Val {
    Int(Int),
    // Numeric ops:
    Sum(Box<Val>, Box<Val>),
    Prod(Box<Val>, Box<Val>),
    Rec(Box<Val>),
    Pow(Box<Val>, Box<Val>),
    // Trig fns:
    Sin(Angle),
    Cos(Angle),
}

#[derive(Clone)]
pub enum Angle {
    Pi(Box<Val>),
    // // Numeric ops:
    Sum(Box<Angle>, Box<Angle>),
    // Trig fns:
    ASin(Box<Val>),
    ACos(Box<Val>),
}

impl Val {
    pub fn from(a: i64) -> Self {
        Self::Int(Int::from(a))
    }

    pub fn add(self, a: Val) -> Self {
        Self::Sum(Box::new(self), Box::new(a))
    }

    pub fn sub(self, a: Val) -> Self {
        self.add(a.neg())
    }

    pub fn neg(self) -> Self {
        self.mul(Self::from(-1))
    }

    pub fn mul(self, a: Val) -> Self {
        Self::Prod(Box::new(self), Box::new(a))
    }

    pub fn div(self, a: Val) -> Self {
        Self::Prod(Box::new(self), Box::new(a.rec()))
    }

    pub fn rec(self) -> Self {
        Self::Rec(Box::new(self))
    }

    pub fn pow(self, a: Val) -> Self {
        Self::Pow(Box::new(self), Box::new(a))
    }

    pub fn sqrt(self) -> Self {
        self.pow(Val::from(1).div(2.into()))
    }

    pub fn pi(self) -> Angle {
        Angle::Pi(Box::new(self))
    }

    pub fn asin(self) -> Angle {
        Angle::ASin(Box::new(self))
    }

    pub fn acos(self) -> Angle {
        Angle::ACos(Box::new(self))
    }
}

impl Angle {
    /// A full turn.
    pub fn turn() -> Self {
        Self::Pi(Box::new(2.into()))
    }

    /// Adds another angle to this one.
    pub fn add(self, a: Angle) -> Angle {
        if let Angle::Pi(x) = self.clone() {
            if let Angle::Pi(y) = a {
                return Angle::Pi(Box::new(x.add(*y)));
            }
        }

        Self::Sum(Box::new(self), Box::new(a))
    }

    pub fn mul(self, a: Val) -> Angle {
        match self {
            Self::Pi(x) => Self::Pi(Box::new(x.mul(a))),
            _ => todo!(),
        }
    }

    pub fn div(self, a: Val) -> Angle {
        match self {
            Self::Pi(x) => Self::Pi(Box::new(x.div(a))),
            _ => todo!(),
        }
    }
}

impl From<i64> for Val {
    fn from(item: i64) -> Self {
        Val::Int(item.into())
    }
}

impl From<i64> for Angle {
    fn from(item: i64) -> Self {
        Angle::Pi(Box::new(item.into()))
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
        None
    }

    fn to_u64(&self) -> Option<u64> {
        None
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
            Val::Int(a) => write!(f, "{}", a),
            Val::Sum(a, b) => write!(f, "({} + {})", a, b),
            Val::Prod(a, b) => write!(f, "({} * {})", a, b),
            Val::Rec(a) => write!(f, "(1 / {})", a),
            Val::Pow(a, b) => write!(f, "{}^{}", a, b),
            Val::Sin(a) => write!(f, "sin({})", a),
            Val::Cos(a) => write!(f, "cos({})", a),
        }
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Angle::Pi(a) => write!(f, "{} * pi", a),
            Self::Sum(a, b) => write!(f, "({} + {})", a, b),
            Angle::ASin(a) => write!(f, "sin^-1({})", a),
            Angle::ACos(a) => write!(f, "cos^-1({})", a),
        }
    }
}

pub fn sqrt(a: Val) -> Val {
    a.sqrt()
}
