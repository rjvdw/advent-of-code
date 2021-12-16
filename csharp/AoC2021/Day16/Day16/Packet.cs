namespace Day16;

public class Packet
{
    private readonly int _version;
    private readonly int _typeId;
    private readonly long? _literalValue;
    private readonly List<Packet> _subPackets;

    public long SumVersions() => _version + _subPackets.Sum(packet => packet.SumVersions());

    public long Eval()
    {
        if (_literalValue.HasValue)
            return _literalValue.Value;

        if (_typeId < 4)
        {
            var subs = _subPackets.Select(packet => packet.Eval());

            return _typeId switch
            {
                0 => subs.Sum(),
                1 => subs.Aggregate(1L, (a, b) => a * b),
                2 => subs.Min(),
                3 => subs.Max(),
                _ => throw new InvalidOperationException($"Invalid type id: {_typeId}"),
            };
        }

        var a = _subPackets[0].Eval();
        var b = _subPackets[1].Eval();

        return _typeId switch
        {
            5 => a > b ? 1 : 0,
            6 => a < b ? 1 : 0,
            7 => a == b ? 1 : 0,
            _ => throw new InvalidOperationException($"Invalid type id: {_typeId}"),
        };
    }

    private Packet(int version, int typeId, long literalValue)
    {
        _version = version;
        _typeId = typeId;
        _literalValue = literalValue;
        _subPackets = new List<Packet>();
    }

    private Packet(int version, int typeId, List<Packet> subPackets)
    {
        _version = version;
        _typeId = typeId;
        _literalValue = null;
        _subPackets = subPackets;
    }

    public static Packet Parse(IEnumerable<string> lines)
    {
        var bits = lines
            .First()
            .SelectMany(MapChar);
        var (_, packet) = Parse(string.Concat(bits), 0);
        return packet;
    }

    private static (int, Packet) Parse(string bits, int position)
    {
        var i = position;
        var version = Convert.ToUInt16(bits[i..(i + 3)], 2);
        i += 3;
        var typeId = Convert.ToUInt16(bits[i..(i + 3)], 2);
        i += 3;

        return typeId == 4
            ? ParseLiteral(bits, i, version, typeId)
            : ParseOperator(bits, i, version, typeId);
    }

    private static (int, Packet) ParseLiteral(string bits, int position, int version, int typeId)
    {
        var i = position;
        var value = "";
        while (i < bits.Length)
        {
            var chunk = bits[i..(i + 5)];
            i += 5;
            value += chunk[1..];
            if (chunk.StartsWith("0"))
                return (i, new Packet(version, typeId, Convert.ToInt64(value, 2)));
        }

        throw new InvalidOperationException("Invalid literal packet");
    }

    private static (int, Packet) ParseOperator(string bits, int position, int version, int typeId)
    {
        var i = position + 1;
        var subPackets = new List<Packet>();

        if (bits[position] == '0')
        {
            var length = Convert.ToInt32(bits[i..(i + 15)], 2);
            i += 15;

            var sub = bits[i..(i + length)];
            i += length;

            for (var s = 0; s < sub.Length;)
            {
                var (np, packet) = Parse(sub, s);
                subPackets.Add(packet);
                s = np;
            }
        }
        else
        {
            var length = Convert.ToInt32(bits[i..(i + 11)], 2);
            i += 11;

            while (subPackets.Count < length)
            {
                var (np, packet) = Parse(bits, i);
                subPackets.Add(packet);
                i = np;
            }
        }

        return (i, new Packet(version, typeId, subPackets));
    }

    private static IEnumerable<char> MapChar(char ch) =>
        ch switch
        {
            '0' => new[] { '0', '0', '0', '0' },
            '1' => new[] { '0', '0', '0', '1' },
            '2' => new[] { '0', '0', '1', '0' },
            '3' => new[] { '0', '0', '1', '1' },
            '4' => new[] { '0', '1', '0', '0' },
            '5' => new[] { '0', '1', '0', '1' },
            '6' => new[] { '0', '1', '1', '0' },
            '7' => new[] { '0', '1', '1', '1' },
            '8' => new[] { '1', '0', '0', '0' },
            '9' => new[] { '1', '0', '0', '1' },
            'A' or 'a' => new[] { '1', '0', '1', '0' },
            'B' or 'b' => new[] { '1', '0', '1', '1' },
            'C' or 'c' => new[] { '1', '1', '0', '0' },
            'D' or 'd' => new[] { '1', '1', '0', '1' },
            'E' or 'e' => new[] { '1', '1', '1', '0' },
            'F' or 'f' => new[] { '1', '1', '1', '1' },
            _ => throw new InvalidOperationException($"Invalid character: {ch}"),
        };
}
