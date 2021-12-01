namespace Day01
{
    // https://adventofcode.com/2021/day/1
    public static class Program
    {
        public static void Main(string[] args)
        {
            if (args.Length != 1)
            {
                Console.Error.WriteLine("Usage: $0 <input file>");
                Environment.Exit(1);
                return;
            }

            var inputFile = args[0];
            var numbers = File
                .ReadAllLines(inputFile)
                .Select(int.Parse)
                .ToList();

            Console.WriteLine(CountIncreases(numbers, 1));
            Console.WriteLine(CountIncreases(numbers, 3));
        }

        public static int CountIncreases(List<int> numbers, int windowSize)
        {
            if (numbers.Count < windowSize)
                return 0;

            var count = 0;
            var window = numbers.Take(windowSize).ToList();
            var index = 0;
            var previousSum = window.Sum();

            foreach (var number in numbers.Skip(windowSize))
            {
                var sum = previousSum - window[index] + number;
                if (sum > previousSum)
                    count += 1;
                window[index] = number;
                index = (index + 1) % windowSize;
                previousSum = sum;
            }

            return count;
        }
    }
}
