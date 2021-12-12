module Solution

open System

[<Literal>]
let A = 0b00000001uy
[<Literal>]
let B = 0b00000010uy
[<Literal>]
let C = 0b00000100uy
[<Literal>]
let D = 0b00001000uy
[<Literal>]
let E = 0b00010000uy
[<Literal>]
let F = 0b00100000uy
[<Literal>]
let G = 0b01000000uy

let countSegments (digit: byte) =
    [A;B;C;D;E;F;G] |> Seq.filter (fun segment -> (digit &&& segment) <> 0uy) |> Seq.length

let getOutput (output: byte seq) (mapping: byte array) =
    let a = mapping.[0]
    let b = mapping.[1]
    let c = mapping.[2]
    let d = mapping.[3]
    let e = mapping.[4]
    let f = mapping.[5]
    let g = mapping.[6]

    let mapper (segments: byte) =
        match segments with
        | x when x = (a ||| b ||| c ||| e ||| f ||| g) -> 0u
        | x when x = (c ||| f) -> 1u
        | x when x = (a ||| c ||| d ||| e ||| g) -> 2u
        | x when x = (a ||| c ||| d ||| f ||| g) -> 3u
        | x when x = (b ||| c ||| d ||| f) -> 4u
        | x when x = (a ||| b ||| d ||| f ||| g) -> 5u
        | x when x = (a ||| b ||| d ||| e ||| f ||| g) -> 6u
        | x when x = (a ||| c ||| f) -> 7u
        | x when x = (a ||| b ||| c ||| d ||| e ||| f ||| g) -> 8u
        | x when x = (a ||| b ||| c ||| d ||| f ||| g) -> 9u
        | _ -> raise (ArgumentException("Invalid output or mapping provided", nameof mapping))

    output
    |> Seq.map mapper
    |> Seq.fold (fun decoded digit -> 10u * decoded + digit) 0u

let countEasyDigits (input: (byte seq * byte seq) seq) =
    input
    |> Seq.collect snd
    |> Seq.map countSegments
    |> Seq.filter (fun count -> count = 2 || count = 3 || count = 4 || count = 7)
    |> Seq.length

let decodeDisplay (digits: byte seq, output: byte seq) =
    let digitsWithCounts =
        digits
        |> Seq.map (fun digit -> (countSegments digit, digit))
        |> List.ofSeq
    let mutable decoded = [| 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy |]
    digitsWithCounts
    |> List.iter
        (fun (count, digit) ->
            match count with
            | 2 -> decoded.[1] <- digit
            | 3 -> decoded.[7] <- digit
            | 4 -> decoded.[4] <- digit
            | 7 -> decoded.[8] <- digit
            | _ -> ())
    digitsWithCounts
    |> List.iter
        (fun (count, digit) ->
            match count with
            | 6 when (digit ||| decoded.[4]) = digit -> decoded.[9] <- digit
            | 6 when (digit ||| decoded.[1]) = digit -> decoded.[0] <- digit
            | 6 -> decoded.[6] <- digit
            | _ -> ())
    digitsWithCounts
    |> List.iter
        (fun (count, digit) ->
            match count with
            | 5 when (digit ||| decoded.[6]) = decoded.[6] -> decoded.[5] <- digit
            | 5 when (digit ||| decoded.[9]) = decoded.[9] -> decoded.[3] <- digit
            | 5 -> decoded.[2] <- digit
            | _ -> ())
    let mutable mapping = [| 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy |]
    mapping.[0] <- decoded.[1] ^^^ decoded.[7] // compare 1 and 7 to find a
    mapping.[2] <- decoded.[5] ^^^ decoded.[9] // compare 5 and 9 to find c
    mapping.[3] <- decoded.[0] ^^^ decoded.[8] // compare 0 and 8 to find d
    mapping.[4] <- decoded.[8] ^^^ decoded.[9] // compare 8 and 9 to find e
    mapping.[1] <- (decoded.[8] - mapping.[4]) ^^^ decoded.[3] // compare (8 - e) and 3 to find b
    mapping.[5] <- (decoded.[8] - mapping.[1]) ^^^ decoded.[2] // compare (8 - b) and 2 to find f
    mapping.[6] <- (decoded.[4] ||| mapping.[0]) ^^^ decoded.[9] // compare (4 + a) and 9 to find g
    getOutput output mapping

let decodeDisplays: (byte seq * byte seq) seq -> uint32 = Seq.map decodeDisplay >> Seq.sum

let parseChar (ch: char) =
    match ch with
    | 'a' -> A
    | 'b' -> B
    | 'c' -> C
    | 'd' -> D
    | 'e' -> E
    | 'f' -> F
    | 'g' -> G
    | _ -> failwith $"Invalid character: {ch}"

let parseDigit (line: string) =
    line
    |> Seq.map parseChar
    |> Seq.fold (fun acc v -> acc ||| v) 0uy

let parse (line: string) =
    let p = line.IndexOf(" | ")
    let digits = line[..p-1].Split(' ') |> Seq.map parseDigit
    let output = line[p+3..].Split(' ') |> Seq.map parseDigit
    (digits, output)
