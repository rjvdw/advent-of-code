open System
open System.IO

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith "Usage: $0 <INPUT FILE>"

let instructions =
    File.ReadLines args.[1]
    |> Seq.map Instruction.parse

let depth1, position1 =
    Solution.computeDepthAndPosition instructions

printfn
    $"Not considering aim, the submarine ends at position {position1} and depth {depth1}, for a final answer of {position1 * depth1}."

let depth2, position2 =
    Solution.computeDepthAndPositionWithAim instructions

printfn
    $"Considering aim, the submarine ends at position {position2} and depth {depth2}, for a final answer of {position2 * depth2}."
