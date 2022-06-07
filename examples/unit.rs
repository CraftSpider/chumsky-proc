
use std::str::FromStr;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Delimiter};
use quote::{quote, ToTokens};
use chumsky_proc::prelude::*;
use chumsky::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum UnitName {
    S,
    M,
    Kg,
    A,
    K,
    Mol,
    Cd,
}

impl UnitName {
    fn parser() -> impl Parser<RustToken, UnitName, Error = Simple<RustToken, RustSpan>> + Clone {
        keyword("s").to(UnitName::S)
            .or(keyword("m").to(UnitName::M))
            .or(keyword("kg").to(UnitName::Kg))
            .or(keyword("A").to(UnitName::A))
            .or(keyword("K").to(UnitName::K))
            .or(keyword("mol").to(UnitName::Mol))
            .or(keyword("cd").to(UnitName::Cd))
    }

    fn eval(&self) -> UnitResult {
        match self {
            UnitName::S => UnitResult { seconds: 1, ..UnitResult::default() },
            UnitName::M => UnitResult { meters: 1, ..UnitResult::default() },
            UnitName::Kg => UnitResult { kilograms: 1, ..UnitResult::default() },
            UnitName::A => UnitResult { amperes: 1, ..UnitResult::default() },
            UnitName::K => UnitResult { kelvin: 1, ..UnitResult::default() },
            UnitName::Mol => UnitResult { mols: 1, ..UnitResult::default() },
            UnitName::Cd => UnitResult { candela: 1, ..UnitResult::default() },
        }
    }
}

#[derive(Debug)]
enum UnitExpr {
    Unit(UnitName),
    Paren(Box<UnitExpr>),
    Pow(Box<UnitExpr>, i8),
    Mul(Box<UnitExpr>, Box<UnitExpr>),
    Div(Box<UnitExpr>, Box<UnitExpr>),
}

impl UnitExpr {
    fn parser() -> impl Parser<RustToken, UnitExpr, Error = Simple<RustToken, RustSpan>> {
        recursive(|expr| {
            let atom = UnitName::parser()
                .map(UnitExpr::Unit)
                .or(
                    expr
                        .delimited_by(just(RustToken::StartDelim(Delimiter::Parenthesis)), just(RustToken::EndDelim(Delimiter::Parenthesis)))
                        .map(|expr| UnitExpr::Paren(Box::new(expr)))
                );

            let pow = atom.then(punct('^').ignore_then(
                punct('-')
                    .or_not()
                    .then(
                        filter_map(|span, tok: RustToken| tok.into_literal()
                            .and_then(|lit: Literal| if let Ok(num) = u8::from_str(&lit.to_string()) {
                                Ok(num)
                            } else {
                                Err(RustToken::Literal(lit))
                            })
                            .map_err(|tok| Simple::expected_input_found(span, [], Some(tok)))
                        )
                    )
                    .map(|(neg, val)| if neg.is_some() {
                        -(val as i8)
                    } else {
                        val as i8
                    })
            ).or_not())
                .map(|(expr, pow)| match pow {
                    Some(pow) => UnitExpr::Pow(Box::new(expr), pow),
                    None => expr,
                });

            let mul = pow.clone().then(pow.repeated())
                .foldl(|left, right| UnitExpr::Mul(Box::new(left), Box::new(right)));

            let div = mul.clone().then(punct('/').ignore_then(mul).repeated())
                .foldl(|left, right| UnitExpr::Div(Box::new(left), Box::new(right)));

            div
        })
    }

    fn eval(&self) -> UnitResult {
        match self {
            UnitExpr::Unit(unit) => {
                unit.eval()
            }
            UnitExpr::Paren(inner) => {
                inner.eval()
            }
            UnitExpr::Pow(left, right) => {
                left.eval().raise_to(*right)
            }
            UnitExpr::Mul(left, right) => {
                let left = left.eval();
                let right = right.eval();

                left.mul(right)
            }
            UnitExpr::Div(left, right) => {
                let left = left.eval();
                let right = right.eval();

                left.div(right)
            }
        }
    }
}

#[derive(Debug, Default)]
struct UnitResult {
    seconds: i8,
    meters: i8,
    kilograms: i8,
    amperes: i8,
    kelvin: i8,
    mols: i8,
    candela: i8,
}

impl UnitResult {
    fn raise_to(self, n: i8) -> UnitResult {
        UnitResult {
            seconds: self.seconds * n,
            meters: self.meters * n,
            kilograms: self.kilograms * n,
            amperes: self.amperes * n,
            kelvin: self.kelvin * n,
            mols: self.mols * n,
            candela: self.candela * n,
        }
    }

    fn mul(self, other: UnitResult) -> UnitResult {
        UnitResult {
            seconds: self.seconds + other.seconds,
            meters: self.meters + other.meters,
            kilograms: self.kilograms + other.kilograms,
            amperes: self.amperes + other.amperes,
            kelvin: self.kelvin + other.kelvin,
            mols: self.mols + other.mols,
            candela: self.candela + other.candela,
        }
    }

    fn div(self, other: UnitResult) -> UnitResult {
        UnitResult {
            seconds: self.seconds - other.seconds,
            meters: self.meters - other.meters,
            kilograms: self.kilograms - other.kilograms,
            amperes: self.amperes - other.amperes,
            kelvin: self.kelvin - other.kelvin,
            mols: self.mols - other.mols,
            candela: self.candela - other.candela,
        }
    }
}

impl ToTokens for UnitResult {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut init = Vec::new();

        if self.seconds != 0 {
            let seconds = self.seconds;
            init.push(quote!(seconds: #seconds));
        }
        if self.meters != 0 {
            let meters = self.meters;
            init.push(quote!(meters: #meters));
        }
        if self.kilograms != 0 {
            let kilograms = self.kilograms;
            init.push(quote!(kilograms: #kilograms));
        }
        if self.amperes != 0 {
            let amperes = self.amperes;
            init.push(quote!(amperes: #amperes));
        }
        if self.kelvin != 0 {
            let kelvin = self.kelvin;
            init.push(quote!(kelvins: #kelvin));
        }
        if self.mols != 0 {
            let mols = self.mols;
            init.push(quote!(mols: #mols));
        }
        if self.candela != 0 {
            let candela = self.candela;
            init.push(quote!(candelas: #candela));
        }

        let out = quote!(units::SiUnit {
            #(
                #init,
            )*
            ..units::SiUnit::unitless()
        });

        tokens.extend(out);
    }
}

/// # Examples
/// ```
/// use unit::Unit;
///
/// let si_unit = Unit![m^2 / kg s^2];
/// ```
#[allow(non_snake_case)]
#[proc_macro]
pub fn Unit(stream: TokenStream) -> TokenStream {
    match UnitExpr::parser().parse(stream_from_tokens(stream.into())) {
        Ok(expr) => expr.eval().into_token_stream().into(),
        Err(errs) => {
            let msg = errs.into_iter()
                .map(|e| format!("{:?}", e))
                .collect::<String>();

            quote!(compile_error!(#msg)).into()
        }
    }
}
