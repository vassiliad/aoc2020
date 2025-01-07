use puzzle::parse::Puzzle;
use puzzle::solve::solve;

use anyhow::Result;

#[test]
fn test_tiny() -> Result<()> {
    let sample = "
abc

a
b
c

ab
ac

a
a
a
a

b
";

    let mut puzzle = Puzzle::from_str(sample)?;

    let answer = solve(&mut puzzle);
    let expected = 6;

    assert_eq!(answer, expected);

    Ok(())
}
