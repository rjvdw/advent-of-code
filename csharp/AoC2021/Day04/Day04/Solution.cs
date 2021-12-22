namespace Day04;

public static class Solution
{
    public static void Solve(IEnumerable<string> input)
    {
        var (numbers, boards) = ParseInput(input);

        var result1 = Play(boards, numbers);
        if (result1 == null)
            Console.Error.WriteLine("No one won the game of squid bingo!");
        else
            Console.WriteLine($"Board #{result1.Value.Idx} won with {result1.Value.LastNumber}, " +
                              $"giving it a score of {result1.Value.Score}, " +
                              $"so the final answer is {result1.Value.LastNumber * result1.Value.Score}.");

        boards.ForEach(board => board.Reset());

        var result2 = FindLosingBoard(boards, numbers);
        if (result2 == null)
            Console.Error.WriteLine("No one won the game of squid bingo!");
        else
            Console.WriteLine($"The losing board is #{result2.Value.Idx}, " +
                              $"which will win with {result2.Value.LastNumber}, " +
                              $"giving it a score of {result2.Value.Score}, " +
                              $"so the final answer is {result2.Value.LastNumber * result2.Value.Score}.");
    }

    public static GameResult? Play(List<Board> boards, List<int> numbers)
    {
        foreach (var number in numbers)
        {
            for (var i = 0; i < boards.Count; i += 1)
            {
                boards[i].Mark(number);
                if (boards[i].Bingo())
                    return new GameResult(i, number, boards[i].Score());
            }
        }

        return null;
    }

    public static GameResult? FindLosingBoard(List<Board> boards, List<int> numbers)
    {
        GameResult? losingBoard = null;
        var ignored = new HashSet<int>();
        foreach (var number in numbers)
        {
            for (var i = 0; i < boards.Count; i += 1)
            {
                if (ignored.Contains(i)) continue;
                boards[i].Mark(number);
                if (boards[i].Bingo())
                {
                    ignored.Add(i);
                    losingBoard = new GameResult(i, number, boards[i].Score());
                }
            }
        }

        return losingBoard;
    }

    public static (List<int>, List<Board>) ParseInput(IEnumerable<string> lines)
    {
        using var enumerator = lines.GetEnumerator();

        // first line contains the numbers
        if (!enumerator.MoveNext())
            throw new ArgumentException("Invalid input", nameof(lines));
        var numbers = enumerator.Current.Split(',').Select(int.Parse).ToList();

        // next line must be empty
        if (!enumerator.MoveNext() || !string.IsNullOrEmpty(enumerator.Current))
            throw new ArgumentException("Invalid input", nameof(lines));

        // remaining lines describe the bingo boards
        var boards = new List<Board>();
        var board = new Board();
        boards.Add(board);
        while (enumerator.MoveNext())
        {
            var line = enumerator.Current;
            if (string.IsNullOrEmpty(line))
            {
                board = new Board();
                boards.Add(board);
                continue;
            }

            var row = line.Split(' ', StringSplitOptions.RemoveEmptyEntries)
                .Select(int.Parse)
                .ToList();
            board.AddRow(row);
        }

        return (numbers, boards);
    }
}
