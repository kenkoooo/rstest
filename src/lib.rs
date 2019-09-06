//! This crate will help you to write simpler tests by leveraging a software testing concept called
//! [test fixtures](https://en.wikipedia.org/wiki/Test_fixture#Software). A fixture is something
//! that you can use in your tests to encapsulate a test's dependencies.
//!
//! The general idea is to have smaller tests that only describe the thing you're testing while you
//! hide the auxiliary utilities your tests make use of somewhere else.
//! For instance, if you have an application that has many tests with users, shopping baskets, and
//! products, you'd have to create a user, a shopping basket, and product every single time in
//! every test which becomes unwieldy quickly. In order to cut down on that repetition, you can
//! instead use fixtures to declare that you need those objects for your function and the fixtures
//! will take care of creating those by themselves. Focus on the important stuff in your tests!
//!
//! In `rstest` a fixture is a function that can return any kind of valid Rust type. This
//! effectively means that your fixtures are not limited by the kind of data they can return.
//! A test can consume an arbitrary number of fixtures at the same time.
//!
//! ## What
//!
//! The `rstest` crate defines the following procedural macros:
//!
//! - [`[rstest]`](attr.rstest.html): A normal Rust test that may additionally take fixtures.
//! - [`[rstest_parametrize]`](attr.rstest_parametrize.html): Like `[rstest]` above but with the
//! added ability to also generate new test cases based on input tables.
//! - [`[rstest_matrix]`](attr.rstest_matrix.html): Like `[rstest]` above but with the
//! added ability to also generate new test cases for every combination of given values.
//! - [`[fixture]`](attr.fixture.html): To mark a function as a fixture.
//!
//! ## Why
//!
//! Very often in Rust we write tests like this
//!
//! ```
//! #[test]
//! fn should_process_two_users() {
//!     let mut repository = create_repository();
//!     repository.add("Bob", 21);
//!     repository.add("Alice", 22);
//!
//!     let processor = string_processor();
//!     processor.send_all(&repository, "Good Morning");
//!
//!     assert_eq!(2, processor.output.find("Good Morning").count());
//!     assert!(processor.output.contains("Bob"));
//!     assert!(processor.output.contains("Alice"));
//! }
//! ```
//!
//! By making use of [`[rstest]`](attr.rstest.html) we can isolate the dependencies `empty_repository` and
//! `string_processor` by passing them as fixtures:
//!
//! ```
//! # use rstest::*;
//! #[rstest]
//! fn should_process_two_users(mut empty_repository: impl Repository,
//!                             string_processor: FakeProcessor) {
//!     empty_repository.add("Bob", 21);
//!     empty_repository.add("Alice", 22);
//!
//!     string_processor.send_all("Good Morning");
//!
//!     assert_eq!(2, string_processor.output.find("Good Morning").count());
//!     assert!(string_processor.output.contains("Bob"));
//!     assert!(string_processor.output.contains("Alice"));
//! }
//! ```
//!
//! ... or if you use `"Alice"` and `"Bob"` in other tests, you can isolate `alice_and_bob` fixture
//! and use it directly:
//!
//! ```
//! # use rstest::*;
//! # trait Repository { fn add(&mut self, name: &str, age: u8); }
//! # struct Rep;
//! # impl Repository for Rep { fn add(&mut self, name: &str, age: u8) {} }
//! # #[fixture]
//! # fn empty_repository() -> Rep {
//! #     Rep
//! # }
//! #[fixture]
//! fn alice_and_bob(mut empty_repository: impl Repository) -> impl Repository {
//!     empty_repository.add("Bob", 21);
//!     empty_repository.add("Alice", 22);
//!     empty_repository
//! }
//!
//! #[rstest]
//! fn should_process_two_users(alice_and_bob: impl Repository,
//!                             string_processor: FakeProcessor) {
//!     string_processor.send_all("Good Morning");
//!
//!     assert_eq!(2, string_processor.output.find("Good Morning").count());
//!     assert!(string_processor.output.contains("Bob"));
//!     assert!(string_processor.output.contains("Alice"));
//! }
//! ```
//!
//! ## Injecting fixtures as function arguments
//!
//! `rstest` functions can receive fixtures by using them as an input argument. A function decorated
//! with [`[rstest]`](attr.rstest.html) will resolve each argument name by call the fixture
//! function. Fixtures should be annotated with the [`[fixture]`](attr.fixture.html) attribute.
//!
//! Fixtures will be resolved like function calls by following the standard resolution rules.
//! Therefore, an identically named fixture can be use in different context.
//!
//! ```
//! # use rstest::*;
//! # trait Repository { }
//! # #[derive(Default)]
//! # struct DataSet {}
//! # impl Repository for DataSet { }
//! mod empty_cases {
//! # use rstest::*;
//! # trait Repository { }
//! # #[derive(Default)]
//! # struct DataSet {}
//! # impl Repository for DataSet { }
//!     use super::*;
//!
//!     #[fixture]
//!     fn repository() -> impl Repository {
//!         DataSet::default()
//!     }
//!
//!     #[rstest]
//!     fn should_do_nothing(repository: impl Repository) {
//!         //.. test impl ..
//!     }
//! }
//!
//! mod non_trivial_case {
//! # use rstest::*;
//! # trait Repository { }
//! # #[derive(Default)]
//! # struct DataSet {}
//! # impl Repository for DataSet { }
//!     use super::*;
//!
//!     #[fixture]
//!     fn repository() -> impl Repository {
//!         let mut ds = DataSet::default();
//!         // Fill your dataset with interesting case
//!         ds
//!     }
//!
//!     #[rstest]
//!     fn should_notify_all_entries(repository: impl Repository) {
//!         //.. test impl ..
//!     }
//! }
//!
//! ```
//!
//! Last but not least, fixtures can be injected like we saw in `alice_and_bob` example.
//!
//! ## Creating parametrized tests
//!
//! You can use use [`[rstest_parametrize]`](attr.rstest_parametrize.html) to create simple
//! table-based tests. Let's see the classic Fibonacci exmple:
//!
//! ```
//! use rstest::rstest_parametrize;
//!
//! #[rstest_parametrize(input, expected,
//!     case(0, 0),
//!     case(1, 1),
//!     case(2, 1),
//!     case(3, 2),
//!     case(4, 3),
//!     case(5, 5),
//!     case(6, 8)
//! )]
//! fn fibonacci_test(input: u32, expected: u32) {
//!     assert_eq!(expected, fibonacci(input))
//! }
//!
//! fn fibonacci(input: u32) -> u32 {
//!     match input {
//!         0 => 0,
//!         1 => 1,
//!         n => fibonacci(n - 2) + fibonacci(n - 1)
//!     }
//! }
//! ```
//! This will generate a bunch of tests, one for every `case()`.


#![cfg_attr(use_proc_macro_diagnostic, feature(proc_macro_diagnostic))]
extern crate proc_macro;

use proc_macro2::{TokenStream, Span};
use syn::{ArgCaptured, FnArg, Generics, Ident, ItemFn, parse_macro_input, Pat, ReturnType, Stmt};

use error::error_statement;
use parse::{Modifiers, RsTestAttribute, RsTestInfo};
use quote::{quote, ToTokens};
use std::iter::FromIterator;
use itertools::Itertools;
use std::collections::HashMap;
use crate::parse::{RsTestItem, FixtureInfo, FixtureItem};

mod parse;
mod error;


trait Tokenize {
    fn into_tokens(self) -> TokenStream;
}

impl<T: ToTokens> Tokenize for T {
    fn into_tokens(self) -> TokenStream {
        quote! { #self }
    }
}

fn default_fixture_resolve(ident: &Ident) -> parse::CaseArg {
    syn::parse2(
        quote! {#ident::default()}
    ).unwrap()
}

fn fn_arg_ident(arg: &FnArg) -> Option<&Ident> {
    match arg {
        FnArg::Captured(ArgCaptured { pat: Pat::Ident(ident), .. }) => Some(&ident.ident),
        _ => None
    }
}

fn arg_2_fixture(ident: &Ident, resolver: &Resolver) -> TokenStream {
    let fixture = resolver
        .resolve(ident)
        .map(|e| e.clone())
        .unwrap_or_else(|| default_fixture_resolve(ident));
    quote! {
        let #ident = #fixture;
    }
}

fn arg_2_fixture_dump(ident: &Ident, modifiers: &RsTestModifiers) -> Option<Stmt> {
    if modifiers.trace_me(ident) {
        syn::parse2(quote! {
            println!("{} = {:?}", stringify!(#ident), #ident);
        }).ok()
    } else {
        None
    }
}

#[derive(Default)]
/// `Resolver` can `resolve` an ident to a `CaseArg`. Pass it to `render_fn_test`
/// function to inject the case arguments resolution.
struct Resolver<'a> {
    borrow: HashMap<String, &'a parse::CaseArg>,
    owned: HashMap<String, parse::CaseArg>,
}

impl<'a> Resolver<'a> {
    fn resolve(&self, ident: &Ident) -> Option<&parse::CaseArg> {
        let ident = ident.to_string();
        self.borrow.get(&ident).map(|&a| a).or_else(|| self.owned.get(&ident))
    }

    fn add_owned(&mut self, ident: &Ident, case_arg: parse::CaseArg) {
        self.owned.insert(ident.to_string(), case_arg);
    }
}

impl<'a> From<(Vec<Ident>, &'a parse::TestCase)> for Resolver<'a> {
    fn from(data: (Vec<Ident>, &'a parse::TestCase)) -> Self {
        let (args, case) = data;
        Self {
            borrow: args.into_iter()
                .zip(case.args.iter())
                .map(|(name,
                          case_arg)| (name.to_string(), case_arg))
                .collect(),
            ..Default::default()
        }
    }
}

impl<'a, ID: ToString> FromIterator<(ID, &'a parse::CaseArg)> for Resolver<'a> {
    fn from_iter<T: IntoIterator<Item=(ID, &'a parse::CaseArg)>>(iter: T) -> Self {
        Self {
            borrow: iter.into_iter()
                .map(|(name, case_arg)| (name.to_string(), case_arg))
                .collect(),
            ..Default::default()
        }
    }
}

fn fn_args(item_fn: &ItemFn) -> impl Iterator<Item=&FnArg> {
    item_fn.decl.inputs.iter()
}

macro_rules! wrap_modifiers {
    ($ident:ident) => {
        #[derive(Default, Debug, PartialEq)]
        pub struct $ident {
            inner: Modifiers,
        }

        impl From<Modifiers> for $ident {
            fn from(inner: Modifiers) -> Self {
                $ident { inner }
            }
        }

        impl $ident {
            fn iter(&self) -> impl Iterator<Item=&RsTestAttribute> {
                self.inner.modifiers.iter()
            }
        }
    };
}

wrap_modifiers!(RsTestModifiers);

impl RsTestModifiers {
    const TRACE_VARIABLE_ATTR: &'static str = "trace";
    const NOTRACE_VARIABLE_ATTR: &'static str = "notrace";

    fn trace_me(&self, ident: &Ident) -> bool {
        if self.should_trace() {
            self.iter()
                .filter(|&m|
                    Self::is_notrace(ident, m)
                ).next().is_none()
        } else { false }
    }

    fn is_notrace(ident: &Ident, m: &RsTestAttribute) -> bool {
        match m {
            RsTestAttribute::Tagged(i, args) if i == Self::NOTRACE_VARIABLE_ATTR =>
                args.iter().find(|&a| a == ident).is_some(),
            _ => false
        }
    }

    fn should_trace(&self) -> bool {
        self.iter()
            .filter(|&m|
                Self::is_trace(m)
            ).next().is_some()
    }

    fn is_trace(m: &RsTestAttribute) -> bool {
        match m {
            RsTestAttribute::Attr(i) if i == Self::TRACE_VARIABLE_ATTR => true,
            _ => false
        }
    }
}

wrap_modifiers!(FixtureModifiers);

impl FixtureModifiers {
    const DEFAULT_RET_ATTR: &'static str = "default";

    fn extract_default_type(self) -> Option<syn::ReturnType> {
        self.iter()
            .filter_map(|m|
                match m {
                    RsTestAttribute::Type(name, t) if name == Self::DEFAULT_RET_ATTR =>
                        Some(syn::parse2(quote!( -> #t)).unwrap()),
                    _ => None
                })
            .next()
    }
}

trait Iterable<I, IT: Iterator<Item=I>, OUT: Iterator<Item=I>> {
    fn iterable(self) -> Option<OUT>;
}

impl<I, IT: Iterator<Item=I>> Iterable<I, IT, std::iter::Peekable<IT>> for IT {
    fn iterable(self) -> Option<std::iter::Peekable<IT>> {
        let mut peekable = self.peekable();
        if peekable.peek().is_some() {
            Some(peekable)
        } else {
            None
        }
    }
}

fn trace_arguments(args: &Vec<Ident>, modifiers: &RsTestModifiers) -> Option<proc_macro2::TokenStream> {
    args.iter()
        .filter_map(move |arg| arg_2_fixture_dump(arg, modifiers))
        .iterable()
        .map(
            |it|
                quote! {
                    println!("{:-^40}", " TEST ARGUMENTS ");
                    #(#it)*
                }
        )
}

fn resolve_args<'a>(args: impl Iterator<Item=&'a Ident>, resolver: &Resolver) -> TokenStream {
    let define_vars = args
        .map(|arg|
            arg_2_fixture(arg, resolver)
        );
    quote! {
        #(#define_vars)*
    }
}

fn resolve_fn_args<'a>(args: impl Iterator<Item=&'a FnArg>, resolver: &Resolver) -> TokenStream {
    resolve_args(args.filter_map(fn_arg_ident), resolver)
}

fn render_fn_test<'a>(name: Ident, testfn: &ItemFn, test_impl: Option<&ItemFn>,
                      resolver: Resolver, modifiers: &'a RsTestModifiers)
                      -> TokenStream {
    let testfn_name = &testfn.ident;
    let args = fn_args_idents(&testfn);
    let attrs = &testfn.attrs;
    let output = &testfn.decl.output;
    let inject = resolve_fn_args(fn_args(&testfn), &resolver);
    let trace_args = trace_arguments(&args, modifiers);
    quote! {
        #[test]
        #(#attrs)*
        fn #name() #output {
            #test_impl
            #(#inject)*
            #trace_args
            println!("{:-^40}", " TEST START ");
            #testfn_name(#(#args),*)
        }
    }
}

fn type_ident(t: &syn::Type) -> Option<&syn::Ident> {
    match t {
        syn::Type::Path(tp) if tp.qself.is_none() && tp.path.segments.len() == 1 => {
            tp.path.segments.first()
                .map(|pair| &pair.value().ident)
        }
        _ => None
    }
}

fn where_predicate_bounded_type(wp: &syn::WherePredicate) -> Option<&syn::Type> {
    match wp {
        syn::WherePredicate::Type(pt) => {
            Some(&pt.bounded_ty)
        }
        _ => None
    }
}

fn generics_clean_up(original: Generics, output: &ReturnType) -> syn::Generics {
    use syn::visit::Visit;
    #[derive(Default, Debug)]
    struct Used(std::collections::HashSet<proc_macro2::Ident>);
    impl<'ast> syn::visit::Visit<'ast> for Used {
        fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
            if i.qself.is_none() && i.path.leading_colon.is_none() && i.path.segments.len() == 1 {
                self.0.insert(i.path.segments.first().unwrap().value().ident.clone());
            }
        }
    }
    let mut outs = Used::default();
    outs.visit_return_type(&output);
    let mut result = original;
    result.params = result.params.into_iter().filter(|p|
        match p {
            syn::GenericParam::Type(tp) if !outs.0.contains(&tp.ident) => false,
            _ => true,
        }
    ).collect();
    result.where_clause.as_mut().map(
        |mut w| w.predicates = w.predicates.clone()
            .into_iter()
            .filter(|wp| where_predicate_bounded_type(wp)
                .and_then(type_ident)
                .map(|t| outs.0.contains(t))
                .unwrap_or(true)
            ).collect()
    );
    result
}

fn render_fixture<'a>(fixture: ItemFn, info: FixtureInfo)
                      -> TokenStream {
    let name = &fixture.ident;
    let vargs = fn_args_idents(&fixture);
    let args = &vargs;
    let orig_args = &fixture.decl.inputs;
    let orig_attrs = &fixture.attrs;
    let generics = &fixture.decl.generics;
    let default_output = info.modifiers.extract_default_type().unwrap_or(fixture.decl.output.clone());
    let default_generics = generics_clean_up(fixture.decl.generics.clone(), &default_output);
    let default_where_clause = &default_generics.where_clause;
    let body = &fixture.block;
    let where_clause = &fixture.decl.generics.where_clause;
    let output = &fixture.decl.output;
    let visibility = &fixture.vis;
    let mut resolver = Resolver::default();
    for f in &info.data.items {
        if let FixtureItem::Fixture(parse::Fixture { ref name, ref positional } ) = f
        {
            let pname = format!("partial_{}", positional.len());
            let partial = Ident::new(&pname, Span::call_site());
            let tokens = quote! {
                    #name::#partial(#(#positional), *)
                };
            resolver.add_owned(name,
                               syn::parse2::<syn::Expr>(tokens)
                                   .expect(&format!("Resolve partial call '{}'", pname))
                                   .into(),
            );
        }
    }
    let inject = resolve_fn_args(fn_args(&fixture), &resolver);
    let partials = (1..=args.len()).map(|n|
        {
            let decl_args = orig_args.iter().cloned().take(n);
            let resolver_args = args.iter().skip(n);
            let inject = resolve_args(resolver_args, &resolver);
            let name = Ident::new(&format!("partial_{}", n), Span::call_site());
            quote! {
                pub fn #name #generics (#(#decl_args),*) #output #where_clause {
                    #(#inject)*
                    Self::get(#(#args),*)
                }
            }
        }
    );
    quote! {
        #[allow(non_camel_case_types)]
        #visibility struct #name {}

        impl #name {
            #(#orig_attrs)*
            pub fn get #generics (#orig_args) #output #where_clause {
                #body
            }

            pub fn default #default_generics () #default_output #default_where_clause {
                #(#inject)*
                Self::get(#(#args),*)
            }

            #(#partials)*
        }

        #[allow(dead_code)]
        #fixture
    }
}

fn fn_args_idents(test: &ItemFn) -> Vec<Ident> {
    fn_args(&test)
        .filter_map(fn_arg_ident)
        .cloned()
        .collect::<Vec<_>>()
}

/// Define a fixture that you can use in all `rstest`'s test arguments. You should just mark your
/// function as `[fixture]` and then use it as a test's argument. Fixture functions can also
/// use other fixtures.
///
/// Let's see a trivial example:
///
/// ```
/// use rstest::*;
///
/// #[fixture]
/// fn twenty_one() -> i32 { 21 }
///
/// #[fixture]
/// fn two() -> i32 { 2 }
///
/// #[fixture]
/// fn injected(twenty_one: i32, two: i32) -> i32 { twenty_one * two }
///
/// #[rstest]
/// fn the_test(injected: i32) {
///     assert_eq!(42, injected)
/// }
/// ```
///
/// You can also partialy inject fixture dependency simply indicate dependency value as fixture
/// argument:
///
/// ```
/// use rstest::*;
///
/// #[fixture]
/// fn base() -> i32 { 1 }
///
/// #[fixture]
/// fn first(base: i32) -> i32 { 1 * base }
///
/// #[fixture]
/// fn second(base: i32) -> i32 { 2 * base }
///
/// #[fixture(second(-3))]
/// fn injected(first: i32, second: i32) -> i32 { first * second }
///
/// #[rstest]
/// fn the_test(injected: i32) {
///     assert_eq!(-6, injected)
/// }
/// ```
/// Note that injected value can be an arbitrary rust expression and not just a literal.
///
/// Sometimes the return type cannot be infered so you must define it: For the few times you may
/// need to do it, you can use the `default<type>` modifier syntax to define it:
///
/// ```
/// use rstest::*;
/// # use std::fmt::Debug;
///
/// #[fixture]
/// pub fn i() -> u32 {
///     42
/// }
///
/// #[fixture(::default<impl Iterator<Item=u32>>)]
/// pub fn fx<I>(i: I) -> impl Iterator<Item=I> {
///     std::iter::once(i)
/// }
///
/// #[rstest]
/// fn resolve<I: Debug + PartialEq>(mut fx: impl Iterator<Item=I>) {
///     assert_eq!(42, fx.next().unwrap())
/// }
/// ```
///
#[proc_macro_attribute]
pub fn fixture(args: proc_macro::TokenStream,
               input: proc_macro::TokenStream)
               -> proc_macro::TokenStream {

    let info: FixtureInfo = parse_macro_input!(args as FixtureInfo);
    let fixture = parse_macro_input!(input as ItemFn);

    if let Some(tokens) = errors_in_fixture(&fixture, &info) {
        tokens
    } else {
        render_fixture(fixture, info).into()
    }.into()
}

/// Write a test that can be injected with [`[fixture]`](attr.fixture.html)s. You can declare all used fixtures
/// by passing them as a function's arguments.
/// ```
/// use rstest::*;
///
/// #[fixture]
/// fn injected() -> i32 { 42 }
///
/// #[rstest]
/// fn the_test(injected: i32) {
///     assert_eq!(42, injected)
/// }
/// ```
///
/// [`[rstest]`](attr.rstest.html) macro will desugar it to something that is not so far from
///
/// ```
/// #[test]
/// fn the_test() {
///     let injected=injected();
///     assert_eq!(42, injected)
/// }
/// ```
///
/// You can dump all input arguments of your test by using the `trace` parameter for the `[rstest]`
/// attribute.
///
/// ```
/// use rstest::*;
///
/// #[fixture]
/// fn injected() -> i32 { 42 }
///
/// #[rstest(::trace)]
/// fn the_test(injected: i32) {
///     assert_eq!(42, injected)
/// }
/// ```
///
/// Will print an output like
///
/// ```bash
/// Testing started at 14.12 ...
/// ------------ TEST ARGUMENTS ------------
/// injected = 42
/// -------------- TEST START --------------
///
///
/// Expected :42
/// Actual   :43
/// ```
/// If you want to trace input arguments but skip some of them that do not implement the `Debug`
/// trait, you can also use the `notrace(list_of_inputs)` modifier:
///
/// ```
/// # use rstest::*;
/// # struct Xyz;
/// # struct NoSense;
/// #[rstest(::trace::notrace(xzy, have_no_sense))]
/// fn the_test(injected: i32, xyz: Xyz, have_no_sense: NoSense) {
///     assert_eq!(42, injected)
/// }
/// ```
#[proc_macro_attribute]
pub fn rstest(args: proc_macro::TokenStream,
              input: proc_macro::TokenStream)
              -> proc_macro::TokenStream {
    let test = parse_macro_input!(input as ItemFn);
    let data: RsTestInfo = parse_macro_input!(args as RsTestInfo);
    let name = &test.ident;
    let mut resolver = Resolver::default();
    for f in &data.data.items {
        if let RsTestItem::Fixture(parse::Fixture { ref name, ref positional } ) = f
        {
            let pname = format!("partial_{}", positional.len());
            let partial = Ident::new(&pname, Span::call_site());
            let tokens = quote! {
                    #name::#partial(#(#positional), *)
                };
            resolver.add_owned(name,
                               syn::parse2::<syn::Expr>(tokens)
                                   .expect(&format!("Resolve partial call '{}'", pname))
                                   .into(),
            );
        }
    }
    if let Some(tokens) = errors_in_rstest(&test, &data) {
        tokens
    } else {
        render_fn_test(name.clone(), &test, Some(&test), resolver, &data.modifiers)
    }.into()
}

fn fn_args_has_ident(fn_decl: &ItemFn, ident: &Ident) -> bool {
    fn_args(fn_decl)
        .filter_map(fn_arg_ident)
        .find(|&id| id == ident)
        .is_some()
}

fn missed_arguments_errors<'a>(test: &'a ItemFn, args: impl Iterator<Item=&'a Ident> + 'a) -> impl Iterator<Item=TokenStream> + 'a {
    args.filter(move |&p| !fn_args_has_ident(test, p))
        .map(|missed|
            error_statement(&format!("Missed argument: '{}' should be a test function argument.", missed),
                            missed.span(), missed.span())
        )
}

fn invalid_case_errors<'a>(params: &'a parse::ParametrizeData) -> impl Iterator<Item=TokenStream> + 'a {
    let n_args = params.args().count();
    params.cases()
        .filter(move |case| case.args.len() != n_args)
        .map(|case|
            error_statement("Wrong case signature: should match the given parameters list.",
                            case.span_start(), case.span_end())
        )
}

fn errors_in_parametrize(test: &ItemFn, info: &parse::ParametrizeInfo) -> Option<TokenStream> {
    let tokens: TokenStream =
        missed_arguments_errors(test, info.data.args())
            .chain(
                invalid_case_errors(&info.data)
            ).collect();

    if !tokens.is_empty() {
        Some(tokens)
    } else {
        None
    }
}

fn errors_in_matrix(test: &ItemFn, info: &parse::MatrixInfo) -> Option<TokenStream> {
    let tokens: TokenStream = missed_arguments_errors(test, info.args.0.iter().map(|v| &v.arg)).collect();

    if !tokens.is_empty() {
        Some(tokens)
    } else {
        None
    }
}

fn errors_in_rstest(test: &ItemFn, info: &parse::RsTestInfo) -> Option<TokenStream> {
    let tokens: TokenStream = missed_arguments_errors(test, info.data.items.iter()
        .map(|v| v.name()))
        .collect();

    if !tokens.is_empty() {
        Some(tokens)
    } else {
        None
    }
}

fn errors_in_fixture(test: &ItemFn, info: &parse::FixtureInfo) -> Option<TokenStream> {
    let tokens: TokenStream = missed_arguments_errors(test, info.data.items.iter()
        .map(|v| v.name()))
        .collect();

    if !tokens.is_empty() {
        Some(tokens)
    } else {
        None
    }
}

struct CaseRender<'a> {
    name: Ident,
    resolver: Resolver<'a>,
}

impl<'a> CaseRender<'a> {
    pub fn new(name: Ident, resolver: Resolver<'a>) -> Self {
        CaseRender { name, resolver }
    }

    fn render(self, testfn: &ItemFn, modifiers: &RsTestModifiers) -> TokenStream {
        render_fn_test(self.name, testfn, None, self.resolver, modifiers)
    }
}

fn render_cases<'a>(test: ItemFn, cases: impl Iterator<Item=CaseRender<'a>>, modifiers: RsTestModifiers) -> TokenStream {
    let fname = &test.ident;
    let cases = cases.map(|case| case.render(&test, &modifiers));

    quote! {
        #[cfg(test)]
        #test

        #[cfg(test)]
        mod #fname {
            use super::*;

            #(#cases)*
        }
    }
}

trait DisplayLen {
    fn display_len(&self) -> usize;
}

impl<D: std::fmt::Display> DisplayLen for D {
    fn display_len(&self) -> usize {
        format!("{}", self).len()
    }
}

fn format_case_name(case: &parse::TestCase, index: usize, display_len: usize) -> String {
    let description = case
        .description.as_ref()
        .map(|d| format!("_{}", d))
        .unwrap_or_default();
    format!("case_{:0len$}{d}", index, len = display_len, d = description)
}

fn render_parametrize_cases(test: ItemFn, params: parse::ParametrizeInfo) -> TokenStream {
    let parse::ParametrizeInfo { data, modifiers } = params;
    let args = data.args().cloned().collect::<Vec<_>>();

    let display_len = data.cases().count().display_len();

    let cases = data.cases()
        .enumerate()
        .map({
            let span = test.ident.span();
            move |(n, case)|
                CaseRender::new(Ident::new(&format_case_name(case, n + 1, display_len), span),
                                (args.clone(), case).into())
        }
        );

    render_cases(test, cases, modifiers.into())
}

fn render_matrix_cases(test: ItemFn, params: parse::MatrixInfo) -> TokenStream {
    let parse::MatrixInfo { args, modifiers } = params;
    let span = test.ident.span();

    // Steps:
    // 1. pack data P=(ident, expr, (pos, max_len)) in one iterator for each variable
    // 2. do a cartesian product of iterators to build all cases (every case is a vector of P)
    // 3. format case by packed data vector
    let cases = args.0.iter()
        .map(|group|
            group.values.iter()
                .enumerate()
                .map(move |(pos, expr)| (&group.arg, expr, (pos, group.values.len())))
        )
        .multi_cartesian_product()
        .map(|c| {
            let args_indexes = c.iter()
                .map(|(_, _, (index, max))|
                    format!("{:0len$}", index + 1, len = max.display_len())
                )
                .collect::<Vec<_>>()
                .join("_");
            let name = format!("case_{}", args_indexes);
            CaseRender::new(Ident::new(&name, span),
                            c.into_iter().map(|(a, e, _)| (a, e)).collect())
        }
        );

    render_cases(test, cases, modifiers.into())
}

/// Write table-based tests: you must indicate the arguments that you want use in your cases
/// and provide them for each case you want to test.
///
/// `rstest_parametrize` generates an independent test for each case.
///
/// ```
/// # use rstest::rstest_parametrize;
/// #[rstest_parametrize(input, expected,
///     case(0, 0),
///     case(1, 1),
///     case(2, 1),
///     case(3, 2),
///     case(4, 3)
/// )]
/// fn fibonacci_test(input: u32, expected: u32) {
///     assert_eq!(expected, fibonacci(input))
/// }
/// ```
///
/// Running `cargo test` in this case executes five tests:
///
/// ```bash
/// running 5 tests
/// test fibonacci_test::case_1 ... ok
/// test fibonacci_test::case_2 ... ok
/// test fibonacci_test::case_3 ... ok
/// test fibonacci_test::case_4 ... ok
/// test fibonacci_test::case_5 ... ok
///
/// test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
/// ```
///
/// Every parameter that isn't mapped in `case()`s will be resolved as `fixture` like
/// [`[rstest]`](attr.rstest.html)'s function arguments.
///
/// In general `rstest_parametrize`'s syntax is:
///
/// ```norun
/// rstest_parametrize(ident_1,..., ident_n,
///     case[::description_1](val_1_1, ..., val_n_1),
///     ...,
///     case[::description_m](val_1_m, ..., val_n_m)[,]
///     [::modifier_1[:: ... [::modifier_k]]]
/// )
/// ```
/// * `ident_x` should be a valid function argument name
/// * `val_x_y` should be a valid rust expression that can be assigned to `ident_x` function argument
/// * `description_l` when present should be a valid Rust identity
/// * modifiers now can be just `trace` or `notrace(args..)` (see [`[rstest]`](attr.rstest.html)
///
/// Functions marked by `rstest_parametrize` can use generics, `impl` and `dyn` without any
/// restriction.
///
/// ```
/// # use rstest::rstest_parametrize;
/// #[rstest_parametrize(input, expected,
///     case("foo", 3),
///     case(String::from("bar"), 3),
/// )]
/// fn len<S: AsRef<str>>(input: S, expected: usize) {
///     assert_eq!(expected, input.as_ref().len())
/// }
///
/// #[rstest_parametrize(input, expected,
///     case("foo", 3),
///     case(String::from("bar"), 3),
/// )]
/// fn len_by_impl(input: impl AsRef<str>, expected: usize) {
///     assert_eq!(expected, input.as_ref().len())
/// }
/// ```
#[proc_macro_attribute]
pub fn rstest_parametrize(args: proc_macro::TokenStream, input: proc_macro::TokenStream)
                          -> proc_macro::TokenStream
{
    let params = parse_macro_input!(args as parse::ParametrizeInfo);
    let test = parse_macro_input!(input as ItemFn);

    if let Some(tokens) = errors_in_parametrize(&test, &params) {
        tokens
    } else {
        render_parametrize_cases(test, params)
    }.into()
}

/// Write matrix-based tests: you must indicate arguments and values list that you want to test and
/// `rstest_matrix` generate an indipendent test for each argument combination (the carthesian
/// product of values lists).
///
/// ```
/// # use rstest::rstest_matrix;
/// #[rstest_matrix(
///     foo => [42, 24],
///     bar => ["foo", "bar"]
/// )]
/// fn matrix_test(foo: u32, bar: &str) {
///     //... test body
/// }
/// ```
///
/// Running `cargo test` in this case executes four tests:
///
/// ```bash
/// running 4 tests
/// test matrix_test::case_1_1 ... ok
/// test matrix_test::case_1_2 ... ok
/// test matrix_test::case_2_1 ... ok
/// test matrix_test::case_2_2 ... ok
///
/// test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
///
/// ```
///
/// Every parameter that isn't mapped as argument list will be resolved as `fixture` like
/// [`[rstest]`](attr.rstest.html)'s function arguments.
///
/// In general `rstest_matrix`'s syntax is:
///
/// ```norun
/// rstest_parametrize(
///     ident_1 => [val_1_1, ..., val_1_m1],
///     ....
///     ident_n => [val_n_1, ..., val_n_mn]
///     [::modifier_1[:: ... [::modifier_k]]]
/// )
/// ```
/// * `ident_x` should be a valid function argument name
/// * `val_x_y` should be a valid rust expression that can be assigned to `ident_x` function argument
/// * modifiers now can be just `trace` or `notrace(args..)` (see [`[rstest]`](attr.rstest.html)
///
/// Functions marked by `rstest_matrix` can use generics, `impl` and `dyn` without any
/// restriction.
///
/// ```
/// # use rstest::rstest_matrix;
/// #[rstest_matrix(
///     foo => ["foo", String::from("foo")]
/// )]
/// fn len<S: AsRef<str>>(foo: S) {
///     assert_eq!(3, input.as_ref().len())
/// }
///
/// #[rstest_matrix(
///     foo => ["foo", String::from("foo")]
/// )]
/// fn len_by_impl(foo: impl AsRef<str>) {
///     assert_eq!(3, input.as_ref().len())
/// }
/// ```
#[proc_macro_attribute]
pub fn rstest_matrix(args: proc_macro::TokenStream, input: proc_macro::TokenStream)
                     -> proc_macro::TokenStream
{
    let info = parse_macro_input!(args as parse::MatrixInfo);
    let test = parse_macro_input!(input as ItemFn);

    if let Some(tokens) = errors_in_matrix(&test, &info) {
        tokens
    } else {
        render_matrix_cases(test, info).into()
    }.into()
}

#[cfg(test)]
mod render {
    use pretty_assertions::assert_eq;
    use unindent::Unindent;
    use syn::{
        export::Debug, Expr, ItemFn, ItemMod, parse::{Parse, ParseStream, Result}, parse2,
        parse_str, punctuated, visit::Visit,
    };

    use crate::parse::*;

    use super::*;

    fn fn_args(item: &ItemFn) -> punctuated::Iter<'_, FnArg> {
        item.decl.inputs.iter()
    }

    fn first_arg_ident(ast: &ItemFn) -> &Ident {
        let arg = fn_args(&ast).next().unwrap();
        fn_arg_ident(arg).unwrap()
    }

    fn assert_syn_eq<P, S>(expected: S, ast: P) where
        S: AsRef<str>,
        P: syn::parse::Parse + Debug + Eq
    {
        assert_eq!(
            parse_str::<P>(expected.as_ref()).unwrap(),
            ast
        )
    }

    fn assert_statement_eq<T, S>(expected: S, tokens: T) where
        T: Into<TokenStream>,
        S: AsRef<str>
    {
        assert_syn_eq::<Stmt, _>(expected, parse2::<Stmt>(tokens.into()).unwrap())
    }

    #[test]
    fn extract_fixture_call_arg() {
        let ast = parse_str("fn foo(mut fix: String) {}").unwrap();
        let arg = first_arg_ident(&ast);
        let resolver = Resolver::default();

        let line = arg_2_fixture(arg, &resolver);

        assert_statement_eq("let fix = fix::default();", line);
    }

    #[test]
    fn extract_fixture_should_not_add_mut() {
        let ast = parse_str("fn foo(mut fix: String) {}").unwrap();
        let arg = first_arg_ident(&ast);
        let resolver = Resolver::default();

        let line = arg_2_fixture(arg, &resolver);

        assert_statement_eq("let fix = fix::default();", line);
    }

    fn case_arg<S: AsRef<str>>(s: S) -> CaseArg {
        parse_str::<Expr>(s.as_ref()).unwrap().into()
    }

    #[test]
    fn arg_2_fixture_str_should_use_passed_fixture_if_any() {
        let ast = parse_str("fn foo(mut fix: String) {}").unwrap();
        let arg = first_arg_ident(&ast);
        let call = case_arg("bar()");
        let mut resolver = Resolver::default();
        resolver.add("fix", &call);

        let line = arg_2_fixture(arg, &resolver);

        assert_statement_eq("let fix = bar();", line);
    }

    impl<'a> Resolver<'a> {
        fn add<S: AsRef<str>>(&mut self, ident: S, expr: &'a CaseArg) {
            self.borrow.insert(ident.as_ref().to_string(), expr);
        }
    }

    #[test]
    fn resolver_should_return_the_given_expression() {
        let ast = parse_str("fn function(mut foo: String) {}").unwrap();
        let arg = first_arg_ident(&ast);
        let expected = case_arg("bar()");
        let mut resolver = Resolver::default();

        resolver.add("foo", &expected);

        assert_eq!(&expected, resolver.resolve(&arg).unwrap())
    }

    #[test]
    fn resolver_should_return_none_for_unknown_argument() {
        let ast = parse_str("fn function(mut fix: String) {}").unwrap();
        let arg = first_arg_ident(&ast);
        let resolver = Resolver::default();

        assert!(resolver.resolve(&arg).is_none())
    }

    mod fn_test_should {
        use pretty_assertions::assert_eq;
        use proc_macro2::Span;

        use super::*;

        #[test]
        fn add_return_type_if_any() {
            let ast: ItemFn = parse_str("fn function(mut fix: String) -> Result<i32, String> { Ok(42) }").unwrap();

            let tokens = render_fn_test(Ident::new("new_name", Span::call_site()),
                                        &ast, None, Default::default(), &Default::default());

            let result: ItemFn = parse2(tokens).unwrap();

            assert_eq!(result.ident.to_string(), "new_name");
            assert_eq!(result.decl.output, ast.decl.output);
        }

        #[test]
        fn should_include_given_function() {
            let input_fn: ItemFn = parse_str(
                r#"
                pub fn test<R: AsRef<str>, B>(mut s: String, v: &u32, a: &mut [i32], r: R) -> (u32, B, String, &str)
                        where B: Borrow<u32>
                {
                    let some = 42;
                    assert_eq!(42, some);
                }
                "#
            ).unwrap();

            let tokens = render_fn_test(Ident::new("new_name", Span::call_site()),
                                        &input_fn, Some(&input_fn), Default::default(), &Default::default());

            let result: ItemFn = parse2(tokens).unwrap();

            let inner_fn: ItemFn = parse2(result.block.stmts.get(0).into_tokens()).unwrap();

            assert_eq!(inner_fn, inner_fn);
        }
    }

    struct TestsGroup {
        requested_test: ItemFn,
        module: ItemMod,
    }

    impl Parse for TestsGroup {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(Self {
                requested_test: input.parse()?,
                module: input.parse()?,
            })
        }
    }

    /// To extract all test functions
    struct TestFunctions(Vec<ItemFn>);

    impl TestFunctions {
        fn is_test_fn(item_fn: &ItemFn) -> bool {
            item_fn.attrs.iter().filter(|&a|
                a.path == parse_str::<syn::Path>("test").unwrap())
                .next().is_some()
        }
    }

    impl<'ast> Visit<'ast> for TestFunctions {
        fn visit_item_fn(&mut self, item_fn: &'ast ItemFn) {
            if Self::is_test_fn(item_fn) {
                self.0.push(item_fn.clone())
            }
        }
    }

    impl TestsGroup {
        pub fn get_test_functions(&self) -> Vec<ItemFn> {
            let mut f = TestFunctions(vec![]);

            f.visit_item_mod(&self.module);
            f.0
        }
    }

    impl From<TokenStream> for TestsGroup {
        fn from(tokens: TokenStream) -> Self {
            syn::parse2::<TestsGroup>(tokens).unwrap()
        }
    }

    mod parametrize_cases {
        use super::{*, assert_eq};

        impl<'a> From<&'a ItemFn> for parse::ParametrizeData {
            fn from(item_fn: &'a ItemFn) -> Self {
                parse::ParametrizeData {
                    data: fn_args_idents(item_fn)
                        .into_iter()
                        .map(ParametrizeItem::CaseArgName)
                        .collect(),
                }
            }
        }

        impl<'a> From<&'a ItemFn> for parse::ParametrizeInfo {
            fn from(item_fn: &'a ItemFn) -> Self {
                parse::ParametrizeInfo {
                    data: item_fn.into(),
                    modifiers: Default::default(),
                }
            }
        }

        #[test]
        fn should_create_a_module_named_as_test_function() {
            let item_fn = parse_str::<ItemFn>("fn should_be_the_module_name(mut fix: String) {}").unwrap();
            let info = (&item_fn).into();
            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let output = TestsGroup::from(tokens);

            assert_eq!(output.module.ident, "should_be_the_module_name");
        }

        #[test]
        fn should_copy_user_function() {
            let item_fn = parse_str::<ItemFn>(
                r#"fn should_be_the_module_name(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let info = (&item_fn).into();
            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let mut output = TestsGroup::from(tokens);

            output.requested_test.attrs = vec![];
            assert_eq!(output.requested_test, item_fn);
        }

        #[test]
        fn should_mark_user_function_as_test() {
            let item_fn = parse_str::<ItemFn>(
                r#"fn should_be_the_module_name(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let info = (&item_fn).into();
            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let output = TestsGroup::from(tokens);

            let expected = parse2::<ItemFn>(quote! {
                #[cfg(test)]
                fn some() {}
            }).unwrap().attrs;

            assert_eq!(expected, output.requested_test.attrs);
        }

        #[test]
        fn should_mark_module_as_test() {
            let item_fn = parse_str::<ItemFn>(
                r#"fn should_be_the_module_name(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let info = (&item_fn).into();
            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let output = TestsGroup::from(tokens);

            let expected = parse2::<ItemMod>(quote! {
                #[cfg(test)]
                mod some {}
            }).unwrap().attrs;

            assert_eq!(expected, output.module.attrs);
        }

        impl ParametrizeInfo {
            fn push_case(&mut self, case: TestCase) {
                self.data.data.push(ParametrizeItem::TestCase(case));
            }

            fn extend(&mut self, cases: impl Iterator<Item=TestCase>) {
                self.data.data.extend(cases.map(ParametrizeItem::TestCase));
            }
        }

        impl<'a> FromIterator<&'a str> for TestCase {
            fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
                TestCase {
                    args: iter.into_iter()
                        .map(CaseArg::from)
                        .collect(),
                    description: None,
                }
            }
        }

        impl<'a> From<&'a str> for TestCase {
            fn from(argument: &'a str) -> Self {
                std::iter::once(argument).collect()
            }
        }

        fn one_simple_case() -> (ItemFn, ParametrizeInfo) {
            let item_fn = parse_str::<ItemFn>(
                r#"fn test(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let mut info: ParametrizeInfo = (&item_fn).into();
            info.push_case(TestCase::from(r#"String::from("3")"#));
            (item_fn, info)
        }

        fn some_simple_cases(cases: i32) -> (ItemFn, ParametrizeInfo) {
            let item_fn = parse_str::<ItemFn>(
                r#"fn test(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let mut info: ParametrizeInfo = (&item_fn).into();
            info.extend((0..cases).map(|_| TestCase::from(r#"String::from("3")"#)));
            (item_fn, info)
        }

        #[test]
        fn should_add_a_test_case() {
            let (item_fn, info) = one_simple_case();

            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let tests = TestsGroup::from(tokens).get_test_functions();

            assert_eq!(1, tests.len());
            assert!(&tests[0].ident.to_string().starts_with("case_"))
        }

        #[test]
        fn case_number_should_starts_from_1() {
            let (item_fn, info) = one_simple_case();

            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let tests = TestsGroup::from(tokens).get_test_functions();

            assert!(&tests[0].ident.to_string().starts_with("case_1"), "Should starts with case_1 but is {}", tests[0].ident.to_string())
        }

        #[test]
        fn should_add_all_test_cases() {
            let (item_fn, info) = some_simple_cases(5);

            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let tests = TestsGroup::from(tokens).get_test_functions();

            let valid_names = tests.iter()
                .filter(|it| it.ident.to_string().starts_with("case_"));
            assert_eq!(5, valid_names.count())
        }

        #[test]
        fn should_left_pad_case_number_by_zeros() {
            let (item_fn, info) = some_simple_cases(1000);

            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let tests = TestsGroup::from(tokens).get_test_functions();

            let first_name = tests[0].ident.to_string();
            let last_name = tests[999].ident.to_string();

            assert!(first_name.ends_with("_0001"), "Should ends by _0001 but is {}", first_name);
            assert!(last_name.ends_with("_1000"), "Should ends by _1000 but is {}", last_name);

            let valid_names = tests.iter()
                .filter(|it| it.ident.to_string().len() == first_name.len());
            assert_eq!(1000, valid_names.count())
        }

        #[test]
        fn should_use_description_if_any() {
            let (item_fn, mut info) = one_simple_case();
            let description = "show_this_description";

            if let &mut ParametrizeItem::TestCase(ref mut case) = &mut info.data.data[1] {
                case.description = Some(parse_str::<Ident>(description).unwrap());
            } else {
                panic!("Test case should be the second one");
            }

            let tokens = render_parametrize_cases(item_fn.clone(), info);

            let tests = TestsGroup::from(tokens).get_test_functions();

            assert!(tests[0].ident.to_string().ends_with(&format!("_{}", description)));
        }
    }

    mod matrix_cases {
        /// Should test matrix tests render without take in account MatrixInfo to ParametrizeInfo
        /// transformation

        use super::{*, assert_eq};

        impl<'a> From<&'a ItemFn> for parse::MatrixValues {
            fn from(item_fn: &'a ItemFn) -> Self {
                parse::MatrixValues(
                    fn_args_idents(item_fn).iter()
                        .map(|it| ValueList { arg: it.clone(), values: vec![] })
                        .collect()
                )
            }
        }

        impl<'a> From<&'a ItemFn> for parse::MatrixInfo {
            fn from(item_fn: &'a ItemFn) -> Self {
                parse::MatrixInfo {
                    args: item_fn.into(),
                    modifiers: Default::default(),
                }
            }
        }

        #[test]
        fn should_create_a_module_named_as_test_function() {
            let item_fn = parse_str::<ItemFn>("fn should_be_the_module_name(mut fix: String) {}").unwrap();
            let info = (&item_fn).into();
            let tokens = render_matrix_cases(item_fn.clone(), info);

            let output = TestsGroup::from(tokens);

            assert_eq!(output.module.ident, "should_be_the_module_name");
        }

        #[test]
        fn should_copy_user_function() {
            let item_fn = parse_str::<ItemFn>(
                r#"fn should_be_the_module_name(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let info = (&item_fn).into();
            let tokens = render_matrix_cases(item_fn.clone(), info);

            let mut output = TestsGroup::from(tokens);

            output.requested_test.attrs = vec![];
            assert_eq!(output.requested_test, item_fn);
        }

        #[test]
        fn should_mark_user_function_as_test() {
            let item_fn = parse_str::<ItemFn>(
                r#"fn should_be_the_module_name(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let info = (&item_fn).into();
            let tokens = render_matrix_cases(item_fn.clone(), info);

            let output = TestsGroup::from(tokens);

            let expected = parse2::<ItemFn>(quote! {
                #[cfg(test)]
                fn some() {}
            }).unwrap().attrs;

            assert_eq!(expected, output.requested_test.attrs);
        }

        #[test]
        fn should_mark_module_as_test() {
            let item_fn = parse_str::<ItemFn>(
                r#"fn should_be_the_module_name(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let info = (&item_fn).into();
            let tokens = render_matrix_cases(item_fn.clone(), info);

            let output = TestsGroup::from(tokens);

            let expected = parse2::<ItemMod>(quote! {
                #[cfg(test)]
                mod some {}
            }).unwrap().attrs;

            assert_eq!(expected, output.module.attrs);
        }

        fn one_simple_case() -> (ItemFn, MatrixInfo) {
            let item_fn = parse_str::<ItemFn>(
                r#"fn test(mut fix: String) { println!("user code") }"#
            ).unwrap();
            let mut info: MatrixInfo = (&item_fn).into();
            info.args.0[0].values.push(CaseArg::from("value".to_string()));
            (item_fn, info)
        }

        #[test]
        fn should_add_a_test_case() {
            let (item_fn, info) = one_simple_case();

            let tokens = render_matrix_cases(item_fn.clone(), info);

            let tests = TestsGroup::from(tokens).get_test_functions();

            assert_eq!(1, tests.len());
            assert!(&tests[0].ident.to_string().starts_with("case_"))
        }

        impl<'a, 'b, 'c, S: ToString> From<(&'a str, &'b [S])> for ValueList {
            fn from(data: (&'a str, &'b [S])) -> Self {
                let (arg, values) = data;
                Self {
                    arg: parse_str(arg).unwrap(),
                    values: values.into_iter().map(|s| CaseArg::from(s.to_string())).collect(),
                }
            }
        }

        #[test]
        fn should_add_a_test_cases_from_all_combinations() {
            let item_fn = parse_str::<ItemFn>(
                r#"fn test(first: u32, second: u32, third: u32) { println!("user code") }"#
            ).unwrap();
            let mut info: MatrixInfo = (&item_fn).into();
            info.args.0[0] = ("first", ["1", "2"].as_ref()).into();
            info.args.0[1] = ("second", ["3", "4"].as_ref()).into();
            info.args.0[2] = ("third", ["5", "6"].as_ref()).into();

            let tokens = render_matrix_cases(item_fn.clone(), info);

            let tests = TestsGroup::from(tokens).get_test_functions();

            let tests = tests.into_iter()
                .map(|t| t.ident.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            assert_eq!(tests, "
                    case_1_1_1
                    case_1_1_2
                    case_1_2_1
                    case_1_2_2
                    case_2_1_1
                    case_2_1_2
                    case_2_2_1
                    case_2_2_2".unindent()
            )
        }

        #[test]
        fn should_pad_case_index() {
            let item_fn = parse_str::<ItemFn>(
                r#"fn test(first: u32, second: u32, third: u32) { println!("user code") }"#
            ).unwrap();
            let mut info: MatrixInfo = (&item_fn).into();
            let values = (1..=100).map(|i| i.to_string()).collect::<Vec<_>>();
            info.args.0[0] = ("first", values.as_ref()).into();
            info.args.0[1] = ("second", values[..10].as_ref()).into();
            info.args.0[2] = ("third", values[..2].as_ref()).into();

            let tokens = render_matrix_cases(item_fn.clone(), info);

            let tests = TestsGroup::from(tokens).get_test_functions();

            assert_eq!(tests[0].ident.to_string(), "case_001_01_1");
            assert_eq!(tests.last().unwrap().ident.to_string(), "case_100_10_2");
        }
    }

    mod fixture {
        use syn::{ItemImpl, ItemStruct};

        use crate::{generics_clean_up, render_fixture};
        use crate::parse::{Modifiers, RsTestAttribute};

        use super::{*, assert_eq};

        #[derive(Clone)]
        struct FixtureOutput {
            orig: ItemFn,
            fixture: ItemStruct,
            core_impl: ItemImpl,
        }

        impl Parse for FixtureOutput {
            fn parse(input: ParseStream) -> Result<Self> {
                Ok(FixtureOutput {
                    fixture: input.parse()?,
                    core_impl: input.parse()?,
                    orig: input.parse()?,
                })
            }
        }

        fn parse_fixture<S: AsRef<str>>(code: S) -> (ItemFn, FixtureOutput) {
            let item_fn = parse_str::<ItemFn>(
                code.as_ref()
            ).unwrap();

            let tokens = render_fixture(item_fn.clone(), Default::default());
            (item_fn, parse2(tokens).unwrap())
        }

        fn test_maintains_function_visibility(code: &str) {
            let (item_fn, out) = parse_fixture(code);

            assert_eq!(item_fn.vis, out.fixture.vis);
            assert_eq!(item_fn.vis, out.orig.vis);
        }

        #[test]
        fn should_maintains_pub_visibility() {
            test_maintains_function_visibility(
                r#"pub fn test() { }"#
            );
        }

        #[test]
        fn should_maintains_no_pub_visibility() {
            test_maintains_function_visibility(
                r#"fn test() { }"#
            );
        }

        fn select_method<S: AsRef<str>>(impl_code: ItemImpl, name: S) -> Option<syn::ImplItemMethod> {
            impl_code.items.into_iter()
                .filter_map(|ii| match ii {
                    syn::ImplItem::Method(f) => Some(f),
                    _ => None
                })
                .find(|f| f.sig.ident == name.as_ref())
        }

        #[test]
        fn fixture_impl_should_implement_a_get_method_with_input_fixture_signature() {
            let (item_fn, out) = parse_fixture(
                r#"
                pub fn test<R: AsRef<str>, B>(mut s: String, v: &u32, a: &mut [i32], r: R) -> (u32, B, String, &str)
                        where B: Borrow<u32>
                { }
                "#);


            let get_decl = select_method(out.core_impl, "get")
                .unwrap()
                .sig
                .decl;

            assert_eq!(*item_fn.decl, get_decl);
        }

        #[test]
        fn fixture_impl_should_implement_a_default_method_with_input_cleaned_fixture_signature_and_no_args() {
            let (item_fn, out) = parse_fixture(
                r#"
                pub fn test<R: AsRef<str>, B, F, H: Iterator<Item=u32>>(mut s: String, v: &u32, a: &mut [i32], r: R) -> (H, B, String, &str)
                        where F: ToString,
                        B: Borrow<u32>

                { }
                "#);

            let default_decl = select_method(out.core_impl, "default")
                .unwrap()
                .sig
                .decl;

            let expected = parse_str::<ItemFn>(
                r#"
                pub fn default<B, H: Iterator<Item=u32>>() -> (H, B, String, &str)
                        where B: Borrow<u32>
                { }
                "#
            ).unwrap();


            assert_eq!(expected.decl.generics, default_decl.generics);
            assert_eq!(item_fn.decl.output, default_decl.output);
            assert!(default_decl.inputs.is_empty());
        }

        #[test]
        fn clean_up_default_generics_no_output() {
            // Should remove all generics parameters that are not present in output
            let item_fn = parse_str::<ItemFn>(
                r#"
                pub fn test<R: AsRef<str>, B, F, H: Iterator<Item=u32>>() -> (H, B, String, &str)
                        where F: ToString,
                        B: Borrow<u32>

                { }
                "#
            ).unwrap();

            let expected = parse_str::<ItemFn>(
                r#"
                pub fn test<B, H: Iterator<Item=u32>>() -> (H, B, String, &str)
                        where B: Borrow<u32>
                { }
                "#
            ).unwrap();

            let cleaned = generics_clean_up(item_fn.decl.generics, &item_fn.decl.output);

            assert_eq!(expected.decl.generics, cleaned);
        }

        #[test]
        fn should_use_default_return_type_if_any() {
            let item_fn = parse_str::<ItemFn>(
                r#"
                pub fn test<R: AsRef<str>, B, F, H: Iterator<Item=u32>>() -> (H, B)
                        where F: ToString,
                        B: Borrow<u32>
                { }
                "#
            ).unwrap();

            let tokens = render_fixture(item_fn.clone(),
                                        FixtureInfo {
                                            modifiers: Modifiers {
                                                modifiers: vec![
                                                    RsTestAttribute::Type(
                                                        parse_str("default").unwrap(),
                                                        parse_str("(impl Iterator<Item=u32>, B)").unwrap(),
                                                    )
                                                ]
                                            }.into(),
                                            ..Default::default()
                                        });
            let out: FixtureOutput = parse2(tokens).unwrap();

            let expected = parse_str::<syn::ItemFn>(
                r#"
                pub fn default<B>() -> (impl Iterator<Item=u32>, B)
                        where B: Borrow<u32>
                { }
                "#
            ).unwrap();

            let default_decl = select_method(out.core_impl, "default")
                .unwrap()
                .sig
                .decl;

            assert_eq!(*expected.decl, default_decl);
        }

        #[test]
        fn should_implement_partial_methods() {
            let (item_fn, out) = parse_fixture(
                r#"
                pub fn test(mut s: String, v: &u32, a: &mut [i32]) -> usize
                { }
                "#);

            let partials = (1..=3).map(|n|
                select_method(out.core_impl.clone(), format!("partial_{}", n))
                    .unwrap()
                    .sig
                    .decl)
                .collect::<Vec<_>>();

            // All 3 methods found

            assert!(select_method(out.core_impl, "partial_4").is_none());

            let expected_1 = parse_str::<ItemFn>(
                r#"
                pub fn partial_1(mut s: String) -> usize
                { }
                "#
            ).unwrap();


            assert_eq!(&expected_1.decl, &Box::new(partials[0].clone()));
            for p in partials {
                assert_eq!(item_fn.decl.output, p.output);
            }
        }
    }
}

