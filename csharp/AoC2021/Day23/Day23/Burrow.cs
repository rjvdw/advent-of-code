namespace Day23;

public record Burrow(List<Amphipod> Amphipods, int SideRoomDepth)
{
    public bool IsFinished() => Amphipods.All(a => a.IsHome());

    public int MinimumRemainingCost() => Amphipods
        .Where(amphipod => !amphipod.IsHome())
        .Sum(amphipod => amphipod.ComputeCost((2, amphipod.Home)));

    public bool TryFindMoveToSideRoom(out Burrow burrow, out int cost)
    {
        throw new NotImplementedException("TODO");
    }

    public IEnumerable<(Burrow Next, int Cost)> FindMovesToHallway()
    {
        throw new NotImplementedException("TODO");
    }

}
