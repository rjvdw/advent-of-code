namespace Day04;

[AttributeUsage(AttributeTargets.Property)]
public class FieldAttribute : Attribute
{
    public string? Prefix { get; init; }
}
