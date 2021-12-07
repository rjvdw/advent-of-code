open System
open System.IO

// https://adventofcode.com/2021/day/1

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith $"Usage: {args[0]} <INPUT FILE>"

let lines = List.ofSeq (File.ReadLines(args[1]))
let numbers = List.map int lines

printfn $"{Solution.countIncreases 1 numbers}"
printfn $"{Solution.countIncreases 3 numbers}"
