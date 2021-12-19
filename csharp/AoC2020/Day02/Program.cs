namespace Day02;

public static class Program
{
    public static void Main(string[] args)
    {
        if (args.Length != 1)
        {
            Console.Error.WriteLine("Usage: $0 <input file>");
            Environment.Exit(1);
            return;
        }

        var inputFile = args[0];
        var entries = File
            .ReadAllLines(inputFile)
            .Select(PasswordEntry.Parse)
            .ToList();

        var nrValidPasswords1 = entries.Count(entry => entry.IsValidAccordingTo1());
        Console.WriteLine($"The solution to part 1 is: {nrValidPasswords1}");

        var nrValidPasswords2 = entries.Count(entry => entry.IsValidAccordingTo2());
        Console.WriteLine($"The solution to part 2 is: {nrValidPasswords2}");
    }
}

public record PasswordEntry(
    int Lower,
    int Upper,
    char Character,
    string Password)
{
    public bool IsValidAccordingTo1()
    {
        var count = Password
            .ToCharArray()
            .Count(ch => ch == Character);

        return count >= Lower && count <= Upper;
    }

    public bool IsValidAccordingTo2()
    {
        var l = Password[Lower - 1] == Character;
        var h = Password[Upper - 1] == Character;
        return (l || h) && !(l && h);
    }

    public static PasswordEntry Parse(string line)
    {
        var parts = line.Split(": ");
        var password = parts[1];
        parts = parts[0].Split(" ");
        var character = char.Parse(parts[1]);
        parts = parts[0].Split("-");
        var lower = int.Parse(parts[0]);
        var upper = int.Parse(parts[1]);

        return new PasswordEntry(lower, upper, character, password);
    }
}
