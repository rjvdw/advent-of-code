using System.Collections.Generic;
using Xunit;

namespace Day12.Test;

public class CaveMapTest
{
    [Fact]
    public void TestCountPaths1()
    {
        var map = CaveMap.Parse(new List<string>
        {
            "start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end",
        });

        Assert.Equal(10, map.CountPaths(0));
        Assert.Equal(36, map.CountPaths(1));
    }

    [Fact]
    public void TestCountPaths2()
    {
        var map = CaveMap.Parse(new List<string>
        {
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN", "kj-dc",
        });

        Assert.Equal(19, map.CountPaths(0));
        Assert.Equal(103, map.CountPaths(1));
    }

    [Fact]
    public void TestCountPaths3()
    {
        var map = CaveMap.Parse(new List<string>
        {
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he", "RW-he", "fs-DX",
            "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        });

        Assert.Equal(226, map.CountPaths(0));
        Assert.Equal(3509, map.CountPaths(1));
    }
}
