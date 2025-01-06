use puzzle::parse::Puzzle;
use puzzle::solve::solve;

use anyhow::Result;
#[test]
fn test_tiny() -> Result<()> {
    let sample = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

    let mut puzzle = Puzzle::from_str(sample)?;

    let answer = solve(&mut puzzle);
    let expected = 1;

    assert_eq!(answer, expected);

    Ok(())
}
