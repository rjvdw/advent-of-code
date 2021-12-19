using Day04;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var (numbers, boards) = Solution.ParseInput(File.ReadLines(args[0]));

var result1 = Solution.Play(boards, numbers);
if (result1 == null)
    Console.Error.WriteLine("No one won the game of squid bingo!");
else
    Console.WriteLine($"Board #{result1.Value.Idx} won with {result1.Value.LastNumber}, " +
                      $"giving it a score of {result1.Value.Score}, " +
                      $"so the final answer is {result1.Value.LastNumber * result1.Value.Score}.");

boards.ForEach(board => board.Reset());

var result2 = Solution.FindLosingBoard(boards, numbers);
if (result2 == null)
    Console.Error.WriteLine("No one won the game of squid bingo!");
else
    Console.WriteLine($"The losing board is #{result2.Value.Idx}, " +
                      $"which will win with {result2.Value.LastNumber}, " +
                      $"giving it a score of {result2.Value.Score}, " +
                      $"so the final answer is {result2.Value.LastNumber * result2.Value.Score}.");
