open System
open System.IO

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith "Usage: $0 <INPUT FILE>"

let input = File.ReadLines args.[1] |> Seq.head
let values = input.Split(',') |> Array.map uint

let point1, cost1 =
    values
    |> Solution.findOptimalPoint Solution.computeFuelNaive

printfn $"Using the naive fuel cost, the optimal point is {point1}, with a cost of {cost1}."

let point2, cost2 =
    values
    |> Solution.findOptimalPoint Solution.computeFuelCorrect

printfn $"Using the correct fuel cost, the optimal point is {point2}, with a cost of {cost2}."
