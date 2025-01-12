use puzzle::parse::Puzzle;
use puzzle::solve::solve;

use anyhow::Result;

#[test]
fn test_tiny() -> Result<()> {
    let sample = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    let mut puzzle = Puzzle::from_str(sample)?;

    println!("{puzzle:?}");

    let answer = solve(&mut puzzle);
    let expected = 8;

    assert_eq!(answer, expected);

    Ok(())
}
