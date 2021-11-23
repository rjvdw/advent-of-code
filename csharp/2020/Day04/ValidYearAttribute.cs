using System.ComponentModel.DataAnnotations;

namespace Day04;

public class ValidYearAttribute : ValidationAttribute
{
    public int Min { get; init; } = int.MinValue;
    public int Max { get; init; } = int.MaxValue;

    public override bool IsValid(object? value)
    {
        if (value == null) return true;

        int year;
        if (value is int i)
            year = i;
        else if (value is string s)
            year = int.Parse(s);
        else
            return false;

        return year >= Min && year <= Max;
    }
}
