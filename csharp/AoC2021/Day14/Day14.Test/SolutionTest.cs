using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using Xunit;

namespace Day14.Test;

using InstructionMap = Dictionary<Pair, (Pair, Pair)>;

public class SolutionTest
{
    [Fact]
    public void TestProcess1()
    {
        var (polymer, instructions) = GetTestData();
        var counts = Solution.Process(polymer, instructions, 1);

        Assert.Equal(2, counts['B']);
        Assert.Equal(2, counts['C']);
        Assert.Equal(1, counts['H']);
        Assert.Equal(2, counts['N']);
    }

    [Fact]
    public void TestProcess2()
    {
        var (polymer, instructions) = GetTestData();
        var counts = Solution.Process(polymer, instructions, 2);

        Assert.Equal(6, counts['B']);
        Assert.Equal(4, counts['C']);
        Assert.Equal(1, counts['H']);
        Assert.Equal(2, counts['N']);
    }

    [Fact]
    public void TestProcess3()
    {
        var (polymer, instructions) = GetTestData();
        var counts = Solution.Process(polymer, instructions, 3);

        Assert.Equal(11, counts['B']);
        Assert.Equal(5, counts['C']);
        Assert.Equal(4, counts['H']);
        Assert.Equal(5, counts['N']);
    }

    [Fact]
    public void TestProcess4()
    {
        var (polymer, instructions) = GetTestData();
        var counts = Solution.Process(polymer, instructions, 4);

        Assert.Equal(23, counts['B']);
        Assert.Equal(10, counts['C']);
        Assert.Equal(5, counts['H']);
        Assert.Equal(11, counts['N']);
    }

    [SuppressMessage("ReSharper", "StringLiteralTypo")]
    private static (string, InstructionMap) GetTestData() =>
        Solution.Parse(new List<string>
        {
            "NNCB",
            "",
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C",
        });
}
