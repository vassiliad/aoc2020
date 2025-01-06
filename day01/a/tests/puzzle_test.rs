use puzzle::parse::Puzzle;
use puzzle::solve::solve;

use anyhow::Result;
#[test]
fn test_tiny() -> Result<()> {
    let sample = "
            1721
            979
            366
            299
            675
            1456";

    let mut puzzle = Puzzle::from_str(sample)?;

    let answer = solve(&mut puzzle);
    let expected = 514579;

    assert_eq!(answer, expected);

    Ok(())
}
