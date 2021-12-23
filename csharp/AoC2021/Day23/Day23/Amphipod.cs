namespace Day23;

public record Amphipod(Color Color, int Index)
{
    public static Amphipod Parse(char ch, int index) =>
        ch switch
        {
            'A' => new Amphipod(Color.Amber, index),
            'B' => new Amphipod(Color.Bronze, index),
            'C' => new Amphipod(Color.Copper, index),
            _ => new Amphipod(Color.Desert, index),
        };

    public int ComputeEnergy(int steps) =>
        steps * Color switch
        {
            Color.Amber => 1,
            Color.Bronze => 10,
            Color.Copper => 100,
            Color.Desert => 1000,
            _ => throw new InvalidOperationException($"Invalid Amphipod: {this}"),
        };

    public int TargetBurrow =>
        Color switch
        {
            Color.Amber => 3,
            Color.Bronze => 5,
            Color.Copper => 7,
            Color.Desert => 9,
            _ => throw new InvalidOperationException($"Invalid Amphipod: {this}"),
        };

    public Amphipod WithIndex(int index) => new(Color, index);
}
