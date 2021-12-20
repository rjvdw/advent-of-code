using Day20;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <steps>");
    Environment.Exit(1);
}

var image = Image.Parse(File.ReadLines(args[0]));
var steps = int.Parse(args[1]);
for (var n = steps; n > 0; n -= 1)
    image = image.Next();

var (nr, isInfinite) = image.CountLitPixels();
Console.WriteLine(isInfinite
    ? $"After {steps} steps, an infinite number of pixels are lit ({nr} of which are within bounds)."
    : $"After {steps} steps, {nr} pixels are lit.");
