namespace Day20;

public static class Solution
{
    public static void Solve(IEnumerable<string> input, int steps)
    {
        var image = Image.Parse(input);
        for (var n = steps; n > 0; n -= 1)
            image = image.Next();

        var (nr, isInfinite) = image.CountLitPixels();
        Console.WriteLine(isInfinite
            ? $"After {steps} steps, an infinite number of pixels are lit ({nr} of which are within bounds)."
            : $"After {steps} steps, {nr} pixels are lit.");
    }
}
