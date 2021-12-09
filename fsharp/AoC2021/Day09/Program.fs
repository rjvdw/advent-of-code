open System
open System.IO

// https://adventofcode.com/2021/day/9

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith $"Usage: {args[0]} <INPUT FILE>"

let dims, map =
    File.ReadLines(args[1]) |> Solution.parse

let lowPoints = map |> Solution.findLowPoints dims

let riskLevels =
    lowPoints
    |> Seq.map (fun idx -> map |> Seq.item idx)
    |> Seq.map (fun v -> (uint v) + 1u)
    |> Seq.sum

printfn $"The sum of the risk levels is {riskLevels}."

let basins =
    lowPoints
    |> Solution.findBasins dims map
    |> Seq.toArray

basins
|> Array.sortInPlaceWith (fun a b -> compare b a)

let threeLargest = basins |> Array.take 3

printf $"The three largest basins have sizes %A{threeLargest}. "
printfn $"The final answer is {threeLargest |> Array.fold (fun a v -> a * v) 1}."
