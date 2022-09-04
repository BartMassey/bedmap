use std::io::{BufRead, Read};

use thiserror::Error;

/// Errors that can be returned by [[bed_map()]].
#[derive(Error, Debug)]
pub enum BedMapError {
    #[error("failed to read range")]
    RangeRead(#[source] std::io::Error),
    #[error("failed to parse range element")]
    RangeParse(#[source] std::num::ParseIntError),
    #[error("bad range format")]
    RangeFormat,
    #[error("failed to read target")]
    TargetRead(#[source] std::io::Error),
}

fn parse_range(range: &str) -> Result<(usize, usize), BedMapError> {
    let elems = range.split('-').fuse();
    let fields: Vec<&str> = elems.take(3).collect();
    if fields.is_empty() || fields[0].is_empty() || fields.len() > 2 {
        return Err(BedMapError::RangeFormat);
    }

    let parse = |s: &str| s.parse().map_err(BedMapError::RangeParse);
    let start = parse(fields[0])?;
    let end = if fields.len() > 1  {
        if fields[1].is_empty() {
            return Err(BedMapError::RangeFormat);
        }
        parse(fields[1])?
    } else {
        start
    };
    if start == 0 || end < start {
        return Err(BedMapError::RangeFormat);
    }

    Ok((start, end + 1))
}

#[test]
fn test_parse_range() {
    assert!(matches!(parse_range("1-3"), Ok((1, 4))));
    assert!(matches!(parse_range("1"), Ok((1, 2))));
    assert!(matches!(parse_range("1-"), Err(BedMapError::RangeFormat)));
    assert!(matches!(parse_range("-1"), Err(BedMapError::RangeFormat)));
    assert!(matches!(parse_range(""), Err(BedMapError::RangeFormat)));
    assert!(matches!(
        parse_range("1-2-3"),
        Err(BedMapError::RangeFormat)
    ));
    assert!(matches!(parse_range("x"), Err(BedMapError::RangeParse(_))));
}

/**
Given a `ranges_source` and a `lines_source`, return an
iterator that produces the lines selected by the ranges.

Ranges are of the form "#-#" or "#", where `#` is a
positive integer. The start of a range can be no greater
than the end; a "#" range selects a single line.
*/
pub fn bed_map<T, U>(
    ranges_source: T,
    lines_source: U,
) -> impl Iterator<Item = Result<String, BedMapError>>
where
    T: Read,
    U: Read,
{
    let fr = std::io::BufReader::new(ranges_source);
    let fl = std::io::BufReader::new(lines_source);

    let mut ranges = fr.lines();
    let mut targets = fl.lines().enumerate().map(|(nline, line)| match line {
        Ok(l) => Ok((nline + 1, l)),
        Err(e) => Err(e),
    });

    let mut cur_range = None;
    let mut cur_line: Option<(usize, String)> = None;

    macro_rules! try_some {
        ($v:expr, $m:expr) => {
            match $v {
                Some(Err(e)) => return Some(Err($m(e))),
                Some(Ok(r)) => Some(r),
                None => None,
            }
        };
        ($v:expr) => {
            match $v {
                Some(Err(e)) => return Some(Err(e)),
                Some(Ok(r)) => Some(r),
                None => None,
            }
        };
    }

    std::iter::from_fn(move || loop {
        if cur_range.is_none() {
            let r = try_some!(ranges.next(), BedMapError::RangeRead)?;
            cur_range = try_some!(Some(parse_range(&r)));
        }
        if cur_line.is_none() {
            cur_line = Some(try_some!(targets.next(), BedMapError::TargetRead)?);
        }
        let (start, end) = cur_range.unwrap();
        let nline = cur_line.as_ref().unwrap().0;
        if nline >= end {
            cur_range = None;
            continue;
        }
        if nline >= start {
            let (_, line) = cur_line.take().unwrap();
            return Some(Ok(line));
        }
        cur_line = None;
    })
}

#[test]
fn test_bed_map() {
    use std::io::Cursor as C;
    let ranges = C::new("2\n4-6\n8\n".to_string());
    let lines = C::new("a\nb\nc\nd\ne\nf\ng\nh\n".to_string());
    let selected: Vec<String> = bed_map(ranges, lines)
        .collect::<Result<Vec<String>, BedMapError>>()
        .unwrap();
    let expected = ["b", "d", "e", "f", "h"].map(str::to_string);
    assert_eq!(selected, expected);
}
