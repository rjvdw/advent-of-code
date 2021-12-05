open System
open System.IO

let argv = Environment.GetCommandLineArgs()

if argv.Length <> 2
then failwith $"Usage: {argv[0]} <INPUT FILE>"

let lines = List.ofSeq(File.ReadLines(argv[1]))
let numbers = List.map int lines

printfn $"{Solution.countIncreases numbers}"
