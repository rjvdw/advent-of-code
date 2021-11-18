namespace Day01
{
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

            var solution1 = SolvePart1(numbers, target);
            if (solution1 == null)
            {
                Console.WriteLine("There is no solution for part 1.");
            }
            else
            {
                Console.WriteLine("The solution to part 1 is: " + solution1);
            }

            var solution2 = SolvePart2(numbers, target);
            if (solution2 == null)
            {
                Console.WriteLine("There is no solution for part 2.");
            }
            else
            {
                Console.WriteLine("The solution to part 2 is: " + solution2);
            }
        }

        public static int? SolvePart1(List<int> numbers, int target)
        {
            for (var i = 0; i < numbers.Count; i += 1)
            {
                for (var j = i + 1; j < numbers.Count; j += 1)
                {
                    if (numbers[i] + numbers[j] == target)
                    {
                        return numbers[i] * numbers[j];
                    }
                }
            }

            return null;
        }

        public static int? SolvePart2(List<int> numbers, int target)
        {
            for (var i = 0; i < numbers.Count; i += 1)
            {
                for (var j = i + 1; j < numbers.Count; j += 1)
                {
                    for (var k = j + 1; k < numbers.Count; k += 1)
                    {
                        if (numbers[i] + numbers[j] + numbers[k] == target)
                        {
                            return numbers[i] * numbers[j] * numbers[k];
                        }
                    }
                }
            }

            return null;
        }
    }
}
