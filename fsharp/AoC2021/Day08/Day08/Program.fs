open System
open System.IO

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith "Usage: $0 <INPUT FILE>"

let input =
    File.ReadLines args.[1] |> Seq.map Solution.parse

printfn $"There are {Solution.countEasyDigits input} easy digits in the output."
printfn $"The sum of all the displays is {Solution.decodeDisplays input}."
