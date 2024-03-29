namespace Day18;

public class SnailNumber
{
    private long? _value;
    private (SnailNumber Left, SnailNumber Right)? _pair;

    /// <summary>
    /// Adds to snail numbers together, and reduces the result to a new valid snail number.
    /// </summary>
    /// <param name="a">The left operand.</param>
    /// <param name="b">The right operand.</param>
    /// <returns>The reduced sum of the two operands.</returns>
    public static SnailNumber operator +(SnailNumber a, SnailNumber b)
    {
        var sn = new SnailNumber { _pair = (a.DeepCopy(), b.DeepCopy()) };

        while (sn.Reduce())
        {
            // noop
        }

        return sn;
    }

    private SnailNumber DeepCopy() => _value.HasValue
        ? new SnailNumber { _value = _value.Value }
        : new SnailNumber { _pair = (_pair!.Value.Left.DeepCopy(), _pair!.Value.Right.DeepCopy()) };

    /// <summary>
    /// The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right
    /// element. The magnitude of a regular number is just that number.
    /// </summary>
    /// <returns>The magnitude of this snail number.</returns>
    public long Magnitude() => _value ?? 3 * _pair!.Value.Left.Magnitude() + 2 * _pair!.Value.Right.Magnitude();

    /// <summary>
    /// <para>
    /// To reduce a snailfish number, you must repeatedly do the first action in this list that applies to the snailfish
    /// number:
    /// </para>
    ///
    /// <list type="bullet">
    /// <item>If any pair is nested inside four pairs, the leftmost such pair explodes.</item>
    /// <item>If any regular number is 10 or greater, the leftmost such regular number splits.</item>
    /// </list>
    ///
    /// <para>Once no action in the above list applies, the snailfish number is reduced.</para>
    ///
    /// <para>
    /// During reduction, at most one action applies, after which the process returns to the top of the list of actions.
    /// For example, if split produces a pair that meets the explode criteria, that pair explodes before other splits
    /// occur.
    /// </para>
    /// </summary>
    /// <returns></returns>
    private bool Reduce() => Explode(0).Exploded || Split();

    /// <summary>
    /// To explode a pair, the pair's left value is added to the first regular number to the left of the exploding pair
    /// (if any), and the pair's right value is added to the first regular number to the right of the exploding pair (if
    /// any). Exploding pairs will always consist of two regular numbers. Then, the entire exploding pair is replaced
    /// with the regular number 0.
    /// </summary>
    /// <param name="depth">The current depth.</param>
    /// <returns>A boolean indicated whether or not some snail number was exploded, and optionally some values that got lost.</returns>
    /// <exception cref="InvalidOperationException"></exception>
    private (bool Exploded, long? Left, long? Right) Explode(int depth)
    {
        if (!_pair.HasValue)
            return (false, null, null);

        return depth switch
        {
            > 3 => throw new InvalidOperationException($"Recursion went to deep ({depth})."),
            < 3 => ExplodeRecurse(depth),
            3 => ExplodeBase(),
        };
    }

    private (bool Exploded, long? Left, long? Right) ExplodeRecurse(int depth)
    {
        var (left, right) = _pair!.Value;
        var (exploded, vLeft, vRight) = left.Explode(depth + 1);
        if (exploded)
        {
            if (vRight.HasValue)
            {
                if (right._value.HasValue)
                    right._value += vRight;
                else
                    right.AddToLeft(vRight.Value);

                return (true, vLeft, null);
            }

            return (true, vLeft, vRight);
        }

        (exploded, vLeft, vRight) = right.Explode(depth + 1);
        if (exploded)
        {
            if (vLeft.HasValue)
            {
                if (left._value.HasValue)
                    left._value += vLeft;
                else
                    left.AddToRight(vLeft.Value);

                return (true, null, vRight);
            }

            return (true, vLeft, vRight);
        }

        return (false, null, null);
    }

    private (bool Exploded, long? Left, long? Right) ExplodeBase()
    {
        var (left, right) = _pair!.Value;
        var explodedPair = left.ExplodePair();
        if (explodedPair.HasValue)
        {
            if (right._value.HasValue)
                right._value += explodedPair.Value.Right;
            else
                right.AddToLeft(explodedPair.Value.Right);
            _pair = (new SnailNumber { _value = 0 }, right);
            return (true, explodedPair.Value.Left, null);
        }

        explodedPair = right.ExplodePair();
        if (explodedPair.HasValue)
        {
            if (left._value.HasValue)
                left._value += explodedPair.Value.Left;
            else
                left.AddToRight(explodedPair.Value.Left);
            _pair = (left, new SnailNumber { _value = 0 });
            return (true, null, explodedPair.Value.Right);
        }

        return (false, null, null);
    }

    /// <summary>
    /// Finds the left most regular value, and adds the <paramref name="value"/>.
    /// </summary>
    /// <param name="value">The value to add.</param>
    private void AddToLeft(long value)
    {
        if (_pair!.Value.Left._value.HasValue)
            _pair!.Value.Left._value += value;
        else
            _pair!.Value.Left.AddToLeft(value);
    }

    /// <summary>
    /// Finds the right most regular value, and adds the <paramref name="value"/>.
    /// </summary>
    /// <param name="value">The value to add.</param>
    private void AddToRight(long value)
    {
        if (_pair!.Value.Right._value.HasValue)
            _pair!.Value.Right._value += value;
        else
            _pair!.Value.Right.AddToRight(value);
    }

    /// <summary>
    /// Returns the values of the children of this snail number.
    /// </summary>
    /// <returns>The values of the children of this snail number, or null if this snail number does not have children.</returns>
    /// <exception cref="InvalidOperationException">Thrown when called on a snail number that has grand children.</exception>
    private (long Left, long Right)? ExplodePair()
    {
        if (!_pair.HasValue)
            return null;
        var (left, right) = _pair.Value;
        if (!left._value.HasValue || !right._value.HasValue)
            throw new InvalidOperationException($"The nesting goes to deep ({this}).");
        return (left._value.Value, right._value.Value);
    }

    /// <summary>
    /// To split a regular number, replace it with a pair; the left element of the pair should be the regular number
    /// divided by two and rounded down, while the right element of the pair should be the regular number divided by two
    /// and rounded up. For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.
    /// </summary>
    /// <returns>True if some number was split, false otherwise.</returns>
    private bool Split()
    {
        if (_value.HasValue)
        {
            var value = _value.Value;
            if (value > 9)
            {
                var left = new SnailNumber { _value = value / 2 };
                var right = new SnailNumber { _value = (value + 1) / 2 };
                _pair = (left, right);
                _value = null;
                return true;
            }

            return false;
        }

        return _pair!.Value.Left.Split() || _pair!.Value.Right.Split();
    }

    /// <summary>
    /// The string representation of a snail number.
    /// </summary>
    /// <returns>The string representation of a snail number.</returns>
    /// <exception cref="InvalidOperationException">Thrown when the snail number is in an invalid state.</exception>
    public override string ToString() => _value.HasValue
        ? _value.Value.ToString()
        : $"[{_pair!.Value.Left},{_pair!.Value.Right}]";

    /// <summary>
    /// Parse a single snail number.
    /// </summary>
    /// <param name="s">The string to parse.</param>
    /// <returns>A snail number.</returns>
    /// <exception cref="InvalidOperationException">Thrown when the snail number cannot be parsed.</exception>
    public static SnailNumber Parse(string s)
    {
        if (!s.StartsWith('['))
            return new SnailNumber { _value = long.Parse(s) };

        var state = new ParsingState();

        for (var i = 0; i < s.Length; i += 1)
        {
            switch (s[i])
            {
                case '[':
                    state.Push();
                    break;
                case ']':
                    state.Pop(s, i);
                    break;
                case ',':
                    if (state.Parsing == ParsingBranch.Left)
                        throw new InvalidOperationException($"Illegal character {s[i]} @ {i}: {s}");
                    break;
                default:
                    state.ParseNumber(s[i]);
                    break;
            }
        }

        if (!state.IsDone())
            throw new InvalidOperationException($"Incomplete input string: {s}");
        return state.Pair.Left;
    }

    private struct ParsingState
    {
        public (SnailNumber Left, SnailNumber Right) Pair = (new SnailNumber(), null!);
        public ParsingBranch Parsing = ParsingBranch.Left;
        private readonly Stack<(ParsingBranch, (SnailNumber Left, SnailNumber Right))> _stack = new();

        public ParsingState()
        {
        }

        public void Push()
        {
            _stack.Push((Parsing, Pair));
            Pair = (new SnailNumber(), new SnailNumber());
            Parsing = ParsingBranch.Left;
        }

        public void Pop(string s, int i)
        {
            if (Parsing == ParsingBranch.Left)
                throw new InvalidOperationException($"Illegal character {s[i]} @ {i}: {s}");

            if (TryPop(out var popped))
            {
                var (parsingPrev, pairPrev) = popped;
                var number = new SnailNumber { _pair = (Pair.Left, Pair.Right) };
                switch (parsingPrev)
                {
                    case ParsingBranch.Left:
                        pairPrev.Left = number;
                        Parsing = ParsingBranch.Right;
                        break;
                    case ParsingBranch.Right:
                        pairPrev.Right = number;
                        break;
                }

                Pair = pairPrev;
            }
            else
                throw new InvalidOperationException($"Illegal character {s[i]} @ {i}: {s}");
        }

        private bool TryPop(out (ParsingBranch parsing, (SnailNumber Left, SnailNumber Right)) p) =>
            _stack.TryPop(out p);

        public void ParseNumber(char c)
        {
            var number = new SnailNumber { _value = c - '0' };
            switch (Parsing)
            {
                case ParsingBranch.Left:
                    Pair.Left = number;
                    Parsing = ParsingBranch.Right;
                    break;
                case ParsingBranch.Right:
                    Pair.Right = number;
                    break;
            }
        }

        public bool IsDone() => _stack.Count == 0;
    }

    /// <summary>
    /// Keep track of whether the left part or the right part of a pair is being parsed.
    /// </summary>
    private enum ParsingBranch
    {
        Left,
        Right,
    }
}
