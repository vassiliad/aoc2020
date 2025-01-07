use puzzle::parse::Puzzle;
use puzzle::solve::solve;

use anyhow::Result;

#[test]
fn test_tiny_0() -> Result<()> {
    let sample = "FBFBBFFRLR";

    let mut puzzle = Puzzle::from_str(sample)?;

    let answer = solve(&mut puzzle);
    let expected = 357;

    assert_eq!(answer, expected);

    Ok(())
}

#[test]
fn test_tiny_1() -> Result<()> {
    let sample = "BFFFBBFRRR";

    let mut puzzle = Puzzle::from_str(sample)?;

    let answer = solve(&mut puzzle);
    let expected = 567;

    assert_eq!(answer, expected);

    Ok(())
}

#[test]
fn test_tiny_2() -> Result<()> {
    let sample = "FFFBBBFRRR";

    let mut puzzle = Puzzle::from_str(sample)?;

    let answer = solve(&mut puzzle);
    let expected = 119;

    assert_eq!(answer, expected);

    Ok(())
}

#[test]
fn test_tiny_3() -> Result<()> {
    let sample = "BBFFBBFRLL";

    let mut puzzle = Puzzle::from_str(sample)?;

    let answer = solve(&mut puzzle);
    let expected = 820;

    assert_eq!(answer, expected);

    Ok(())
}
