namespace Day04
{
    // https://adventofcode.com/2020/day/4
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
            var passports = File
                .ReadAllText(inputFile)
                .Replace('\n', ' ')
                .TrimEnd()
                .Split("  ")
                .Select(Passport.Parse)
                .ToList();

            Console.WriteLine($"The solution to part 1 is: {passports.Count(p => p.HasAllRequiredFields())}");
            Console.WriteLine($"The solution to part 2 is: {passports.Count(p => p.IsValid())}");
        }
    }
}
