namespace Day08;

public record Display(byte[] Digits, byte[] Output)
{
    private const string Separator = " | ";

    public const byte A = 0b00000001;
    public const byte B = 0b00000010;
    public const byte C = 0b00000100;
    public const byte D = 0b00001000;
    public const byte E = 0b00010000;
    public const byte F = 0b00100000;
    public const byte G = 0b01000000;

    private static readonly byte[] AllSegments = { A, B, C, D, E, F, G };

    public uint GetOutput(byte[] mapping)
    {
        var a = mapping[0];
        var b = mapping[1];
        var c = mapping[2];
        var d = mapping[3];
        var e = mapping[4];
        var f = mapping[5];
        var g = mapping[6];

        var decoded = (uint)0;
        foreach (var segments in Output)
        {
            var digit = segments switch
            {
                var x when x == (a | b | c | e | f | g) => (uint)0,
                var x when x == (c | f) => (uint)1,
                var x when x == (a | c | d | e | g) => (uint)2,
                var x when x == (a | c | d | f | g) => (uint)3,
                var x when x == (b | c | d | f) => (uint)4,
                var x when x == (a | b | d | f | g) => (uint)5,
                var x when x == (a | b | d | e | f | g) => (uint)6,
                var x when x == (a | c | f) => (uint)7,
                var x when x == (a | b | c | d | e | f | g) => (uint)8,
                var x when x == (a | b | c | d | f | g) => (uint)9,
                _ => throw new ArgumentException("Invalid mapping provided", nameof(mapping))
            };
            decoded *= 10;
            decoded += digit;
        }

        return decoded;
    }

    public static int CountSegments(byte digit) => AllSegments.Count(segment => (digit & segment) != 0);

    public static Display Parse(string input)
    {
        var idx = input.IndexOf(Separator, StringComparison.Ordinal);
        if (idx == -1) throw new ArgumentException("Invalid input provided", nameof(input));

        var digits = input[..idx].Split(' ').Select(MapStrToBytes).ToArray();
        var output = input[(idx + Separator.Length)..].Split(' ').Select(MapStrToBytes).ToArray();

        return new Display(digits, output);
    }

    private static byte MapStrToBytes(string input)
    {
        var bytes = (byte)0;
        foreach (var ch in input)
            bytes |= ch switch
            {
                'a' => A,
                'b' => B,
                'c' => C,
                'd' => D,
                'e' => E,
                'f' => F,
                'g' => G,
                _ => 0
            };
        return bytes;
    }
}
