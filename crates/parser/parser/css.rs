// auto-generated: "lalrpop 0.20.2"
// sha3: 8b6718a507956b05ffe6a844868c8ad41b472013bb2d3c8488e2556cdfa8a714
use std::str::FromStr;
use lalrpop_util::ErrorRecovery;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::never_loop, clippy::match_single_binding, clippy::needless_raw_string_hashes)]
mod __parse__Term {

    use std::str::FromStr;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(i32),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 5,
        // State 1
        2, 0, 5,
        // State 2
        0, -2, 0,
        // State 3
        0, 0, 0,
        // State 4
        0, -1, 0,
        // State 5
        0, 7, 0,
        // State 6
        0, -3, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 3 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -2,
        // State 3
        -4,
        // State 4
        -1,
        // State 5
        0,
        // State 6
        -3,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 2,
            1 => match state {
                1 => 5,
                _ => 3,
            },
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###"r#"[0-9]+"#"###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'err,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    where
        'input: 'err,
        'static: 'err,
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'err>
    where 'input: 'err, 'static: 'err
    {
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err> __state_machine::ParserDefinition for __StateMachine<'input, 'err>
    where 'input: 'err, 'static: 'err
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = i32;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 3 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.errors,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(1, _) if true => Some(0),
            Token(2, _) if true => Some(1),
            Token(0, _) if true => Some(2),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 => match __token {
                Token(1, __tok0) | Token(2, __tok0) | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err>>
    where
        'input: 'err,
        'static: 'err,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 1,
                }
            }
            3 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct TermParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for TermParser { fn default() -> Self { Self::new() } }
    impl TermParser {
        pub fn new() -> TermParser {
            let __builder = super::__intern_token::new_builder();
            TermParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
            input: &'input str,
        ) -> Result<i32, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    errors,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    where
        'input: 'err,
        'static: 'err,
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<i32,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                // __Term = Term => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[0-9]+"# => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    fn __reduce1<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Num => ActionFn(1);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
    }
    fn __reduce2<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Term, ")" => ActionFn(2);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action2::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Term::TermParser;
#[rustfmt::skip]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("[0-9]+", false),
            ("\\(", false),
            ("\\)", false),
            (r"\s+", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, n, _): (usize, i32, usize),
) -> i32
{
    n
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
) -> i32
{
    t
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(s).unwrap()
}
#[allow(clippy::type_complexity, dead_code)]

pub  trait __ToTriple<'input, 'err, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
