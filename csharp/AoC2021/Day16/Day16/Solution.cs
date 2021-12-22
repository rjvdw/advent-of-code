namespace Day16;

public static class Solution
{
    public static void Solve(IEnumerable<string> input)
    {
        var packet = Packet.Parse(input);

        Console.WriteLine($"The sum of the versions is {packet.SumVersions()}.");
        Console.WriteLine($"The transmission evaluates to {packet.Eval()}.");
    }
}
