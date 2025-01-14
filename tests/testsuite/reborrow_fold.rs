#![allow(dead_code)]
// #![allow(unused_variables)]

use std::str;

use winnow::bytes::take_till1;
use winnow::multi::fold_many0;
use winnow::prelude::*;
use winnow::sequence::delimited;
use winnow::IResult;

fn atom(_tomb: &mut ()) -> impl for<'a> FnMut(&'a [u8]) -> IResult<&'a [u8], String> {
    move |input| {
        take_till1(" \t\r\n")
            .map_res(str::from_utf8)
            .map(ToString::to_string)
            .parse_next(input)
    }
}

// FIXME: should we support the use case of borrowing data mutably in a parser?
fn list<'a>(i: &'a [u8], tomb: &mut ()) -> IResult<&'a [u8], String> {
    delimited(
        '(',
        fold_many0(atom(tomb), String::new, |mut acc: String, next: String| {
            acc.push_str(next.as_str());
            acc
        }),
        ')',
    )
    .parse_next(i)
}
