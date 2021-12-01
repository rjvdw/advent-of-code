using System.ComponentModel.DataAnnotations;
using System.Reflection;
using System.Text;

namespace Day04;

public class Passport
{
    [Field(Prefix = "byr"), Required, ValidYear(Min = 1920, Max = 2002)]
    public string? BirthYear { get; set; }

    [Field(Prefix = "iyr"), Required, ValidYear(Min = 2010, Max = 2020)]
    public string? IssueYear { get; set; }

    [Field(Prefix = "eyr"), Required, ValidYear(Min = 2020, Max = 2030)]
    public string? ExpirationYear { get; set; }

    [Field(Prefix = "hgt"), Required, ValidHeight(MinCm = 150, MaxCm = 193, MinInch = 59, MaxInch = 76)]
    public string? Height { get; set; }

    [Field(Prefix = "hcl"), Required, RegularExpression(@"^#[0-9a-f]{6}$")]
    public string? HairColor { get; set; }

    [Field(Prefix = "ecl"), Required, RegularExpression(@"^(amb|blu|brn|gry|grn|hzl|oth)$")]
    public string? EyeColor { get; set; }

    [Field(Prefix = "pid"), Required, RegularExpression(@"^\d{9}$")]
    public string? PassportId { get; set; }

    [Field(Prefix = "cil")] public string? CountryId { get; set; }

    public static Passport Parse(string line)
    {
        var passport = new Passport();
        var properties = GetFields();

        foreach (var entry in line.Split(' '))
        {
            var value = entry[4..];
            foreach (var (property, field) in properties)
                if (field.Prefix != null && entry.StartsWith(field.Prefix))
                    property.SetValue(passport, value);
        }

        return passport;
    }

    private record FieldProperty(PropertyInfo Property, FieldAttribute Attribute);

    private static IEnumerable<PropertyInfo> GetProperties() =>
        typeof(Passport).GetProperties(BindingFlags.Instance | BindingFlags.NonPublic | BindingFlags.Public);

    private static List<FieldProperty> GetFields() => GetProperties()
        .Select(property => (property, Attribute.GetCustomAttribute(property, typeof(FieldAttribute))))
        .Where(tuple => tuple.Item2 != null)
        .Select(tuple => new FieldProperty(tuple.Item1, (FieldAttribute)tuple.Item2!))
        .ToList();

    public bool HasAllRequiredFields() => GetProperties()
        .Where(property => Attribute.GetCustomAttribute(property, typeof(RequiredAttribute)) != null)
        .All(property => property.GetValue(this) != null);

    public bool IsValid() => Validator.TryValidateObject(
        this,
        new ValidationContext(this, null, null),
        null,
        true);

    public override string ToString()
    {
        var sb = new StringBuilder();
        foreach (var (property, field) in GetFields())
        {
            var value = property.GetValue(this);
            if (value != null)
            {
                sb.Append(field.Prefix);
                sb.Append(':');
                sb.Append(value);
                sb.Append(' ');
            }
        }

        return sb.ToString().TrimEnd();
    }
};
