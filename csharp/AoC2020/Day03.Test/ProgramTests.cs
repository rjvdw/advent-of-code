using Xunit;

namespace Day03.Test;

public class ProgramTests
{
    [Fact]
    public void Test1()
    {
        Assert.Equal(2, GetTrack().CountTrees(1, 1));
    }

    [Fact]
    public void Test2()
    {
        Assert.Equal(7, GetTrack().CountTrees(3, 1));
    }

    [Fact]
    public void Test3()
    {
        Assert.Equal(3, GetTrack().CountTrees(5, 1));
    }

    [Fact]
    public void Test4()
    {
        Assert.Equal(4, GetTrack().CountTrees(7, 1));
    }

    [Fact]
    public void Test5()
    {
        Assert.Equal(2, GetTrack().CountTrees(1, 2));
    }

    private static Track GetTrack()
    {
        return new Track(new[]
        {
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        });
    }
}
