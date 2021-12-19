using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace Day19.Test;

public class SolutionTest
{
    [Fact]
    public void TestFindBeacons()
    {
        var scanners = ScannerTest.GetTestScanners().ToList();
        var corrected = Solution.CorrectScanners(scanners);
        var beacons = Solution.FindBeacons(corrected).ToList();
        beacons.Sort();
        Assert.Equal(AllBeacons(), beacons);
    }

    [Fact]
    public void TestMaxDistance()
    {
        var scanners = ScannerTest.GetTestScanners().ToList();
        var corrected = Solution.CorrectScanners(scanners).ToList();
        Assert.Equal(3621, Solution.MaxDistance(corrected));
    }

    private static List<Point> AllBeacons()
    {
        var points = new List<Point>
        {
            new(-892, 524, 684),
            new(-876, 649, 763),
            new(-838, 591, 734),
            new(-789, 900, -551),
            new(-739, -1745, 668),
            new(-706, -3180, -659),
            new(-697, -3072, -689),
            new(-689, 845, -530),
            new(-687, -1600, 576),
            new(-661, -816, -575),
            new(-654, -3158, -753),
            new(-635, -1737, 486),
            new(-631, -672, 1502),
            new(-624, -1620, 1868),
            new(-620, -3212, 371),
            new(-618, -824, -621),
            new(-612, -1695, 1788),
            new(-601, -1648, -643),
            new(-584, 868, -557),
            new(-537, -823, -458),
            new(-532, -1715, 1894),
            new(-518, -1681, -600),
            new(-499, -1607, -770),
            new(-485, -357, 347),
            new(-470, -3283, 303),
            new(-456, -621, 1527),
            new(-447, -329, 318),
            new(-430, -3130, 366),
            new(-413, -627, 1469),
            new(-345, -311, 381),
            new(-36, -1284, 1171),
            new(-27, -1108, -65),
            new(7, -33, -71),
            new(12, -2351, -103),
            new(26, -1119, 1091),
            new(346, -2985, 342),
            new(366, -3059, 397),
            new(377, -2827, 367),
            new(390, -675, -793),
            new(396, -1931, -563),
            new(404, -588, -901),
            new(408, -1815, 803),
            new(423, -701, 434),
            new(432, -2009, 850),
            new(443, 580, 662),
            new(455, 729, 728),
            new(456, -540, 1869),
            new(459, -707, 401),
            new(465, -695, 1988),
            new(474, 580, 667),
            new(496, -1584, 1900),
            new(497, -1838, -617),
            new(527, -524, 1933),
            new(528, -643, 409),
            new(534, -1912, 768),
            new(544, -627, -890),
            new(553, 345, -567),
            new(564, 392, -477),
            new(568, -2007, -577),
            new(605, -1665, 1952),
            new(612, -1593, 1893),
            new(630, 319, -379),
            new(686, -3108, -505),
            new(776, -3184, -501),
            new(846, -3110, -434),
            new(1135, -1161, 1235),
            new(1243, -1093, 1063),
            new(1660, -552, 429),
            new(1693, -557, 386),
            new(1735, -437, 1738),
            new(1749, -1800, 1813),
            new(1772, -405, 1572),
            new(1776, -675, 371),
            new(1779, -442, 1789),
            new(1780, -1548, 337),
            new(1786, -1538, 337),
            new(1847, -1591, 415),
            new(1889, -1729, 1762),
            new(1994, -1805, 1792),
        };
        points.Sort();
        return points;
    }
}
