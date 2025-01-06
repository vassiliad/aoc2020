use puzzle::parse::Puzzle;
use puzzle::solve::solve;

use anyhow::Result;
#[test]
fn test_tiny() -> Result<()> {
    let sample = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    let mut puzzle = Puzzle::from_str(sample)?;

    let answer = solve(&mut puzzle);
    let expected = 336;

    assert_eq!(answer, expected);

    Ok(())
}
