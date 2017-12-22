#[macro_use]
extern crate nom;

named!(
    pixel<bool>,
    alt!(value!(true, tag!("#")) | value!(false, tag!(".")))
);

named!(pixels<Vec<bool>>, many1!(pixel));

named!(
    grid<Vec<Vec<bool>>>,
    separated_list_complete!(tag!("/"), pixels)
);

named!(
    input_row<(Vec<Vec<bool>>, Vec<Vec<bool>>)>,
    do_parse!(pattern: grid >> tag!(" => ") >> replace: grid >> (pattern, replace))
);

pub fn parse<T>(input: &str, mapper: fn((Vec<Vec<bool>>, Vec<Vec<bool>>)) -> T) -> Vec<T> {
    input
        .lines()
        .map(|line| input_row(line.as_bytes()).to_result().unwrap())
        .map(mapper)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;

    const EMPTY: &[u8] = b"";

    #[test]
    fn test_pixel() {
        assert_eq!(pixel(b"#"), Done(EMPTY, true));
        assert_eq!(pixel(b"."), Done(EMPTY, false));
    }

    #[test]
    fn test_pixels() {
        assert_eq!(pixels(b".#"), Done(EMPTY, vec![false, true]));
    }

    #[test]
    fn test_grid() {
        assert_eq!(
            grid(b"../.#"),
            Done(EMPTY, vec![vec![false, false], vec![false, true]])
        );
    }

    #[test]
    fn test_input_row() {
        assert_eq!(
            input_row(b"../.# => ##./#../..."),
            Done(
                EMPTY,
                (
                    vec![vec![false, false], vec![false, true]],
                    vec![
                        vec![true, true, false],
                        vec![true, false, false],
                        vec![false, false, false],
                    ]
                )
            )
        );
    }
}
