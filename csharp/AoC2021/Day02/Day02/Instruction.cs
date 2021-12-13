namespace Day02;

public readonly struct Instruction
{
    public readonly Direction Direction;
    public readonly int Distance;

    private Instruction(Direction direction, int distance)
    {
        Direction = direction;
        Distance = distance;
    }

    public static Instruction Parse(string s)
    {
        var p = s.IndexOf(' ');
        if (p == -1)
            throw new ArgumentException($"Invalid input: {s}.");
        var distance = int.Parse(s[(p + 1)..]);
        return s[..p] switch
        {
            "forward" => new Instruction(Direction.Forward, distance),
            "down" => new Instruction(Direction.Down, distance),
            "up" => new Instruction(Direction.Up, distance),
            _ => throw new ArgumentException($"Invalid input: {s}."),
        };
    }
}
