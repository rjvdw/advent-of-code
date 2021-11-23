using System.ComponentModel.DataAnnotations;
using System.Text.RegularExpressions;

namespace Day04;

public class ValidHeightAttribute : ValidationAttribute
{
    public int MinInch { get; init; } = int.MinValue;
    public int MaxInch { get; init; } = int.MaxValue;
    public int MinCm { get; init; } = int.MinValue;
    public int MaxCm { get; init; } = int.MaxValue;

    public override bool IsValid(object? value)
    {
        if (value == null) return true;
        if (value is not string strValue) return false;
        var match = Regex.Match(strValue, @"\D");
        if (!match.Success) return false;
        var height = int.Parse(strValue[..match.Index]);

        if (strValue.EndsWith("in"))
            return height >= MinInch && height <= MaxInch;
        if (strValue.EndsWith("cm"))
            return height >= MinCm && height <= MaxCm;

        return false;
    }
}
