using Xunit;

namespace Day20.Test;

public class ImageTest
{
    [Fact]
    public void TestNextAndCountLitPixels()
    {
        var image = GetTestImage();
        Assert.Equal((10, false), image.CountLitPixels());
        image = image.Next();
        Assert.Equal((24, false), image.CountLitPixels());
        image = image.Next();
        Assert.Equal((35, false), image.CountLitPixels());
    }

    private static Image GetTestImage() => Image.Parse(new[]
    {
        string.Join("",
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##",
            "#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###",
            ".######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.",
            ".#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....",
            ".#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..",
            "...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....",
            "..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"),
        "",
        "#..#.",
        "#....",
        "##..#",
        "..#..",
        "..###",
    });
}
