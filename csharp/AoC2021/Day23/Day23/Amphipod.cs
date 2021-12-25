namespace Day23;

public record Amphipod(AmphipodColor Color, bool Exhausted, Node Location)
{
    public static Amphipod Parse(char ch, Node location) => new(
        AmphipodColorExtensions.Parse(ch),
        false,
        location
    );

    public int ComputeCost(Node to) => Color.Cost() * Location.DistanceTo(to);

    public int Home() => Color.Home();

    public bool IsHome() => Color.Home() == Location.X;

    public bool IsInSideRoom() => Location.IsSideRoom();

    public Amphipod WithLocation(Node location) => new(
        Color,
        location.IsSideRoom(),
        location
    );
}

public enum AmphipodColor
{
    Amber,
    Bronze,
    Copper,
    Desert,
}

internal static class AmphipodColorExtensions
{
    internal static AmphipodColor Parse(char ch) => ch switch
    {
        'A' => AmphipodColor.Amber,
        'B' => AmphipodColor.Bronze,
        'C' => AmphipodColor.Copper,
        _ => AmphipodColor.Desert,
    };

    internal static int Cost(this AmphipodColor color) => color switch
    {
        AmphipodColor.Amber => 1,
        AmphipodColor.Bronze => 10,
        AmphipodColor.Copper => 100,
        AmphipodColor.Desert => 1000,
        _ => throw new ArgumentOutOfRangeException(nameof(color), color, "Invalid color"),
    };

    internal static int Home(this AmphipodColor color) => color switch
    {
        AmphipodColor.Amber => 3,
        AmphipodColor.Bronze => 5,
        AmphipodColor.Copper => 7,
        AmphipodColor.Desert => 9,
        _ => throw new ArgumentOutOfRangeException(nameof(color), color, "Invalid color"),
    };
}
