use nom::{
    bytes::complete::take_while1,
    character::complete::{char, space0},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub fn balanced_parens(input: &str) -> IResult<&str, &str> {
    let mut depth = 1; // start after the opening '('
    let mut end = 0;
    let chars = input.char_indices();
    for (i, c) in chars {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    if depth == 0 {
        Ok((&input[end + 1..], &input[..end]))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

pub fn func_call(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = space0(input)?;
    let (input, func_name) = take_while1(|c: char| c.is_alphabetic() || c == '_')(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, args_str) = balanced_parens(input)?;
    let (input, _) = space0(input)?;
    Ok((input, (func_name, args_str)))
}

pub fn parse_pipe(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        delimited(space0, char('|'), space0),
        take_while1(|c| c != '|'),
    )(input)
}
