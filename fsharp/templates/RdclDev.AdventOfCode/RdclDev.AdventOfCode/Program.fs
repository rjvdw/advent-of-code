open System
open System.IO

// https://adventofcode.com/<yyyy>/day/<d>

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith "Usage: $0 <INPUT FILE>"

let lines = File.ReadLines args.[1]

printfn $"{Solution.solve lines}"
