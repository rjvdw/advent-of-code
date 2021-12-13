namespace Day10;

public class Analysis
{
    public AnalysisType Type { get; }
    private readonly char? _invalidCharacter;
    private readonly string? _missingCharacters;

    public char InvalidCharacter
    {
        get
        {
            if (Type != AnalysisType.Invalid)
                throw new InvalidOperationException("Cannot get invalid character of valid line.");
            return _invalidCharacter!.Value;
        }
    }

    public string MissingCharacters
    {
        get
        {
            if (Type != AnalysisType.Valid)
                throw new InvalidOperationException("Cannot get missing characters of invalid line.");
            return _missingCharacters!;
        }
    }

    private Analysis(AnalysisType type, char? invalidCharacter, string? missingCharacters)
    {
        Type = type;
        _invalidCharacter = invalidCharacter;
        _missingCharacters = missingCharacters;
    }

    public ulong Score() => Type switch
    {
        AnalysisType.Invalid => _invalidCharacter switch
        {
            ')' => 3UL,
            ']' => 57UL,
            '}' => 1197UL,
            '>' => 25137,
            _ => throw new InvalidOperationException($"illegal character: {_invalidCharacter}"),
        },
        AnalysisType.Valid => _missingCharacters!
            .ToCharArray()
            .Select(ch => ch switch
            {
                ')' => 1UL,
                ']' => 2UL,
                '}' => 3UL,
                '>' => 4UL,
                _ => throw new InvalidOperationException($"illegal character: {ch}"),
            })
            .Aggregate(0UL, (acc, v) => 5 * acc + v),
        _ => throw new InvalidOperationException($"Invalid validation result: %A{this}"),
    };

    private static Analysis Valid(string missingCharacters) => new(AnalysisType.Valid, null, missingCharacters);

    private static Analysis Invalid(char invalidCharacter) => new(AnalysisType.Invalid, invalidCharacter, null);

    public static Analysis Parse(string line)
    {
        var stack = new Stack<char>();

        foreach (var ch in line)
        {
            switch (ch)
            {
                case '(' or '[' or '{' or '<':
                    stack.Push(ch);
                    break;
                case ')' or ']' or '}' or '>' when stack.Count == 0:
                    return Invalid(ch);
                case ')' or ']' or '}' or '>':
                    switch (stack.Pop())
                    {
                        case '(' when ch == ')':
                            break;
                        case '[' when ch == ']':
                            break;
                        case '{' when ch == '}':
                            break;
                        case '<' when ch == '>':
                            break;
                        default:
                            return Invalid(ch);
                    }

                    break;
                default:
                    throw new ArgumentException($"Invalid character ({ch}) encountered in line: {line}");
            }
        }

        var missing = string.Concat(stack
            .Select(ch => ch switch
            {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => throw new InvalidOperationException(),
            }));

        return Valid(missing);
    }

    public enum AnalysisType
    {
        Invalid,
        Valid,
    }
}
