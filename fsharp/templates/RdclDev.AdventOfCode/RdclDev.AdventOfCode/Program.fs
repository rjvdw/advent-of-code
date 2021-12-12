open System
open System.IO

// https://adventofcode.com/......

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith "Usage: $0 <INPUT FILE>"

let lines = List.ofSeq (File.ReadLines(args[1]))

printfn $"{Solution.solve lines}"
