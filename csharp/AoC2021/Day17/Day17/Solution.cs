namespace Day17;

public static class Solution
{
    public static void Solve(IEnumerable<string> input)
    {
        var targetArea = TargetArea.Parse(input);
        Console.WriteLine($"The maximal height that can be reached is {targetArea.FindMaxHeight()}.");
        Console.WriteLine($"There are {targetArea.FindAllValidTrajectories().Count()} possible initial velocities.");
    }
}
