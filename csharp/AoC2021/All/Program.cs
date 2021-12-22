if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file dir>");
    Environment.Exit(1);
}

IEnumerable<string> ReadInput(int day) => File.ReadAllLines(Path.Join(args[0], $"day{day:D2}.txt"));

Day01.Solution.Solve(ReadInput(1), 1);
Day01.Solution.Solve(ReadInput(1), 3);
Day02.Solution.Solve(ReadInput(2));
Day03.Solution.Solve(ReadInput(3));
Day04.Solution.Solve(ReadInput(4));
Day05.Solution.Solve(ReadInput(5));
Day06.Solution.Solve(ReadInput(6), 80);
Day06.Solution.Solve(ReadInput(6), 256);
Day07.Solution.Solve(ReadInput(7));
Day08.Solution.Solve(ReadInput(8));
Day09.Solution.Solve(ReadInput(9));
Day10.Solution.Solve(ReadInput(10));
Day11.Solution.Solve(ReadInput(11), 100);
Day11.Solution.Solve(ReadInput(11));
Day12.Solution.Solve(ReadInput(12), 0);
Day12.Solution.Solve(ReadInput(12), 1);
Day13.Solution.Solve(ReadInput(13));
Day14.Solution.Solve(ReadInput(14), 10);
Day14.Solution.Solve(ReadInput(14), 40);
Day15.Solution.Solve(ReadInput(15));
Day16.Solution.Solve(ReadInput(16));
Day17.Solution.Solve(ReadInput(17));
Day18.Solution.Solve(ReadInput(18));
Day19.Solution.Solve(ReadInput(19));
Day20.Solution.Solve(ReadInput(20), 2);
Day20.Solution.Solve(ReadInput(20), 50);
Day21.Solution.Solve(ReadInput(21), "part1", 1000);
Day21.Solution.Solve(ReadInput(21), "part2", 21);
Day22.Solution.Solve(ReadInput(22));
