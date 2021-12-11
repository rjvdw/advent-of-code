namespace Day02;

public static class Solution
{
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
            }
        }

        return (depth, position);
    }
}
