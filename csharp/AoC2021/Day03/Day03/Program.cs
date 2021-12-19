using Day03;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var len = 0;
var values = File
    .ReadAllLines(args[0])
    .Select(line =>
    {
        len = line.Length;
        return Convert.ToUInt16(line, 2);
    })
    .ToList();

var (gamma, epsilon) = Solution.ComputeGammaAndEpsilonRate(values, len);
Console.WriteLine($"The gamma rate is {gamma}, and the epsilon rate is {epsilon}, " +
                  $"so the final answer is {gamma * epsilon}");

var rating = Solution.ComputeLifeSupportRating(values, len);
if (rating == null)
    Console.Error.WriteLine("No life support rating could be determined.");
else
    Console.WriteLine($"The life support rating is {rating}.");
