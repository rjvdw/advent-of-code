﻿using System.Diagnostics.CodeAnalysis;

namespace Day09;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var heightMap = HeightMap.Parse(input);
        var lowPoints = heightMap.FindLowPoints().ToList();
        var values = lowPoints.Select(p => heightMap[p]);
        Console.WriteLine($"The sum of the risk levels is {CalculateRisk(values)}.");

        var basins = heightMap.FindBasins(lowPoints);
        var threeLargest = basins.OrderByDescending(s => s).Take(3).ToList();
        Console.WriteLine($"The three largest basins have sizes [{string.Join(", ", threeLargest)}]. " +
                          $"The final answer is {threeLargest.Aggregate((a, v) => a * v)}.");

    }

    public static uint CalculateRisk(IEnumerable<byte> values) =>
        values.Select(v => 1u + v).Aggregate((a, v) => a + v);
}
