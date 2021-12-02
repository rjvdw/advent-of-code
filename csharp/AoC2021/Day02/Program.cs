namespace Day02
{
    // https://adventofcode.com/2021/day/1
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
            var instructions = File
                .ReadAllLines(inputFile)
                .Select(Instruction.Parse)
                .ToList();

            var (depth1, position1) = FollowInstructions(instructions);
            Console.WriteLine($"Not considering aim, the submarine ends at position {position1} and depth {depth1}, " +
                              $"for a final answer of {position1 * depth1}.");

            var (depth2, position2) = FollowInstructionsWithAim(instructions);
            Console.WriteLine($"Considering aim, the submarine ends at position {position2} and depth {depth2}, " +
                              $"for a final answer of {position2 * depth2}.");
        }

        public static (int, int) FollowInstructions(List<Instruction> instructions)
        {
            var depth = 0;
            var position = 0;

            foreach (var instruction in instructions)
            {
                switch (instruction.Direction)
                {
                    case Direction.Forward:
                        position += instruction.Distance;
                        break;
                    case Direction.Up:
                        depth -= instruction.Distance;
                        break;
                    case Direction.Down:
                        depth += instruction.Distance;
                        break;
                    default:
                        throw new ArgumentException("Invalid enum value for direction", nameof(instruction.Direction));
                }
            }

            return (depth, position);
        }

        public static (int, int) FollowInstructionsWithAim(List<Instruction> instructions)
        {
            var aim = 0;
            var depth = 0;
            var position = 0;

            foreach (var instruction in instructions)
            {
                switch (instruction.Direction)
                {
                    case Direction.Forward:
                        position += instruction.Distance;
                        depth += instruction.Distance * aim;
                        break;
                    case Direction.Up:
                        aim -= instruction.Distance;
                        break;
                    case Direction.Down:
                        aim += instruction.Distance;
                        break;
                    default:
                        throw new ArgumentException("Invalid enum value for direction", nameof(instruction.Direction));
                }
            }

            return (depth, position);
        }
    }
}
