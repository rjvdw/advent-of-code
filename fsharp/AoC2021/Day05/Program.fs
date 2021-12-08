open System
open System.IO

// https://adventofcode.com/2021/day/5

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith $"Usage: {args[0]} <INPUT FILE>"

let lines = File.ReadLines(args[1]) |> List.ofSeq |> List.map Solution.Line.parse

printfn $"Not considering diagonals, there are {Solution.countDangerousPoints false lines} points where multiple lines overlap."
printfn $"Considering diagonals, there are {Solution.countDangerousPoints true lines} points where multiple lines overlap."
