namespace Day25;

public enum SeaCucumber
{
    East,
    South,
    Empty,
}

public static class SeaCucumberExtensions
{
    public static bool IsEast(this SeaCucumber sc) => sc == SeaCucumber.East;
    public static bool IsEmpty(this SeaCucumber sc) => sc == SeaCucumber.Empty;
}
