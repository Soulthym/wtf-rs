type Bool<Then, Else, Output> = Box<dyn FnOnce(Then, Else) -> Output>;

pub fn btrue<Then, Else>(br_then: Then, _: Else) -> Then {
    br_then
}
pub fn bfalse<Then, Else>(_: Then, br_else: Else) -> Else {
    br_else
}

pub fn bif<Then, Else, Output>(cond: Bool<Then, Else, Output>, br_then: Then, br_else: Else) -> Output {
    cond(br_then, br_else)
}

pub fn bnot<Then, Else, Output>(bool: Bool<Then, Else, Output>) -> Bool<Else, Then, Output>
where
    Then: 'static,
    Else: 'static,
    Output: 'static,
{
    Box::new(move |br_else, br_then| bool(br_then, br_else))
}

pub fn band<Then, Else, Output, Tmp>(
    lhs: Bool<Tmp, Else, Output>,
    rhs: Bool<Then, Else, Tmp>,
) -> Bool<Then, Else, Output>
where
    Tmp: 'static,
    Then: 'static,
    Else: 'static + Clone,
    Output: 'static,
{
    Box::new(move |br_then: Then, br_else: Else| lhs(rhs(br_then, br_else.clone()), br_else))
}

pub fn bor<Then, Else, Output, Tmp>(
    lhs: Bool<Then, Tmp, Output>,
    rhs: Bool<Then, Else, Tmp>,
) -> Bool<Then, Else, Output>
where
    Tmp: 'static,
    Then: 'static + Clone,
    Else: 'static,
    Output: 'static,
{
    Box::new(move |br_then: Then, br_else: Else| lhs(br_then.clone(), rhs(br_then, br_else)))
}

pub fn to_string<Output>(bool: Bool<String, String, Output>) -> Output {
    bif(bool, "true".to_string(), "false".to_string())
}

pub fn example<Output>(bool: Bool<isize, String, Output>) -> Output {
    bif(bool, 5, "hi!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(example(Box::new(btrue)), 5);
        assert_eq!(example(Box::new(bfalse)), "hi!".to_string());
    }
}
