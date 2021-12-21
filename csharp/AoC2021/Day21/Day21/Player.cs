namespace Day21;

public class Player
{
    private int Position { get; }
    public int Score { get; }

    private Player(int position, int score)
    {
        Position = position;
        Score = score;
    }

    public Player Move(int roll)
    {
        var position = (Position + roll) % 10;
        var score = Score + position + 1;
        return new Player(position, score);
    }

    public bool HasWon(int targetScore) => Score >= targetScore;

    public static Player Parse(string line)
    {
        var i = line.LastIndexOf(' ');
        if (i == -1) throw new FormatException($"Invalid player: {line}.");
        return new Player(int.Parse(line[(i + 1)..]) - 1, 0);
    }
}
