open System
open System.IO

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith "Usage: $0 <INPUT FILE>"

let results =
    File.ReadLines args.[1]
    |> Seq.map Solution.parse
    |> Seq.toList

let invalid_score =
    results
    |> List.choose
        (fun result ->
            match result with
            | Solution.Invalid _ -> Some(Solution.score result)
            | _ -> None)
    |> List.sum

printfn $"The total score of all invalid lines is {invalid_score}."

let valid_score =
    results
    |> List.choose
        (fun result ->
            match result with
            | Solution.Valid _ -> Some(Solution.score result)
            | _ -> None)
    |> List.sort
    |> List.toArray

printfn $"The middle score of all valid lines is {valid_score.[valid_score.Length / 2]}."
