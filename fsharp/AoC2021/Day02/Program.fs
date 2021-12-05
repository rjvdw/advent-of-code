open System
open System.IO

// https://adventofcode.com/2021/day/2

let args = Environment.GetCommandLineArgs()

if args.Length <> 2
then failwith $"Usage: {args[0]} <INPUT FILE>"

let lines = List.ofSeq(File.ReadLines(args[1]))
let instructions = List.map Instruction.parse lines

let depth1, position1 = Solution.computeDepthAndPosition instructions
printfn $"Not considering aim, the submarine ends at position {position1} and depth {depth1}, for a final answer of {position1 * depth1}."

let depth2, position2 = Solution.computeDepthAndPositionWithAim instructions
printfn $"Considering aim, the submarine ends at position {position2} and depth {depth2}, for a final answer of {position2 * depth2}."
