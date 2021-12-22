namespace Day22;

public static class Solution
{
    private static readonly Cuboid InitializationRegion = new(
        false, // irrelevant
        new Range(-50, 50),
        new Range(-50, 50),
        new Range(-50, 50)
    );

    public static void Solve(IEnumerable<string> input)
    {
        var cubes = input.Select(Cuboid.Parse);

        var (init, full) = ExecuteRebootSequence(cubes);

        Console.WriteLine($"After the initialization sequence, {init} cubes are on.");
        Console.WriteLine($"After the full reboot sequence, {full} cubes are on.");
    }

    public static (long InitializationSequenceCount, long TotalCount) ExecuteRebootSequence(IEnumerable<Cuboid> cuboids)
    {
        var initDone = false;
        var initCount = 0L;
        var onRegions = new List<Cuboid>();

        foreach (var cuboid in cuboids)
        {
            if (!initDone && !cuboid.FitsWithin(InitializationRegion))
            {
                initDone = true;
                initCount = CountCubes(onRegions);
            }

            var nextOnRegions = new List<Cuboid>();

            foreach (var region in onRegions)
            {
                if (region.IsDisjoint(cuboid))
                    nextOnRegions.Add(region);
                else
                    nextOnRegions.AddRange(region - cuboid);
            }

            if (cuboid.IsOn)
                nextOnRegions.Add(cuboid);

            onRegions = nextOnRegions;
        }

        if (!initDone)
            initCount = CountCubes(onRegions);

        return (initCount, CountCubes(onRegions));
    }

    private static long CountCubes(IEnumerable<Cuboid> cuboids) => cuboids.Select(cuboid => cuboid.Size()).Sum();
}
