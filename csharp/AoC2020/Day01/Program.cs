namespace Day01;

// https://adventofcode.com/2020/day/1
public static class Program
{
    public static void Main(string[] args)
    {
        if (args.Length != 2)
        {
            Console.Error.WriteLine("Usage: $0 <input file> <target>");
            Environment.Exit(1);
            return;
        }

        var inputFile = args[0];
        var numbers = File
            .ReadAllLines(inputFile)
            .Select(int.Parse)
            .ToList();
        var target = int.Parse(args[1]);

        var solution1 = FindNumbers(numbers, target, 2);
        Console.WriteLine($"The solution to part 1 is: {solution1?.ToString() ?? "n/a"}");

        var solution2 = FindNumbers(numbers, target, 3);
        Console.WriteLine($"The solution to part 2 is: {solution2?.ToString() ?? "n/a"}");
    }

    public static int? FindNumbers(List<int> numbers, int target, int count, int offset = 0)
    {
        switch (count)
        {
            case < 1:
                return null;
            case 1:
                return numbers.Contains(target) ? target : null;
        }

        var upper = numbers.Count + 1 - count;
        for (var i = offset; i < upper; i += 1)
        {
            var nr = numbers[i];
            if (target <= nr) continue;
            var result = FindNumbers(numbers, target - nr, count - 1, i + 1);
            if (result != null)
                return result * nr;
        }

        return null;
    }
}
