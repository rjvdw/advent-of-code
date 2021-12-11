namespace Day08;

public static class Solution
{
    public static int CountEasyDigits(IEnumerable<Display> displays) =>
        displays
            .Select(display => display.Output)
            .SelectMany(output => output
                .Select(Display.CountSegments))
            .Count(count => count is 2 or 3 or 4 or 7);

    public static uint DecodeDisplays(IEnumerable<Display> displays) =>
        displays
            .Select(Decode)
            .Aggregate(0U, (a, v) => a + v);

    public static uint Decode(Display display)
    {
        List<(int Count, byte Digit)> digitsWithCounts = display
            .Digits
            .Select(digit => (Display.CountSegments(digit), digit))
            .ToList();

        byte[] digits = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
        foreach (var (count, digit) in digitsWithCounts)
        {
            if (count == 2)
                digits[1] = digit;
            else if (count == 3)
                digits[7] = digit;
            else if (count == 4)
                digits[4] = digit;
            else if (count == 7)
                digits[8] = digit;
        }

        foreach (var (_, digit) in digitsWithCounts.Where(entry => entry.Count == 6))
        {
            if ((digit | digits[4]) == digit)
                digits[9] = digit;
            else if ((digit | digits[1]) == digit)
                digits[0] = digit;
            else
                digits[6] = digit;
        }

        foreach (var (_, digit) in digitsWithCounts.Where(entry => entry.Count == 5))
        {
            if ((digit | digits[6]) == digits[6])
                digits[5] = digit;
            else if ((digit | digits[9]) == digits[9])
                digits[3] = digit;
            else
                digits[2] = digit;
        }

        byte[] mapping = { 0, 0, 0, 0, 0, 0, 0 };
        mapping[0] = (byte)(digits[1] ^ digits[7]); // compare 1 and 7 to find a
        mapping[2] = (byte)(digits[5] ^ digits[9]); // compare 5 and 9 to find c
        mapping[3] = (byte)(digits[0] ^ digits[8]); // compare 0 and 8 to find d
        mapping[4] = (byte)(digits[8] ^ digits[9]); // compare 8 and 9 to find e
        mapping[1] = (byte)((digits[8] - mapping[4]) ^ digits[3]); // compare (8 - e) and 3 to find b
        mapping[5] = (byte)((digits[8] - mapping[1]) ^ digits[2]); // compare (8 - b) and 2 to find f
        mapping[6] = (byte)((digits[4] | mapping[0]) ^ digits[9]); // compare (4 + a) and 9 to find g

        return display.GetOutput(mapping);
    }
}
