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

let getOutput (output: seq<byte>) (mapping: list<byte>) =
    let a = mapping[0]
    let b = mapping[1]
    let c = mapping[2]
    let d = mapping[3]
    let e = mapping[4]
    let f = mapping[5]
    let g = mapping[6]

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

let countEasyDigits (input: list<seq<byte> * seq<byte>>) =
    input
    |> Seq.collect snd
    |> Seq.map countSegments
    |> Seq.filter (fun count -> count = 2 || count = 3 || count = 4 || count = 7)
    |> Seq.length

let decodeDisplay (digits: seq<byte>, output: seq<byte>) =
    let digitsWithCounts =
        digits
        |> Seq.map (fun digit -> (countSegments digit, digit))
        |> List.ofSeq
    let mutable decoded = [| 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy |]
    digitsWithCounts
    |> List.iter
        (fun (count, digit) ->
            match count with
            | 2 -> decoded[1] <- digit
            | 3 -> decoded[7] <- digit
            | 4 -> decoded[4] <- digit
            | 7 -> decoded[8] <- digit
            | _ -> ())
    digitsWithCounts
    |> List.iter
        (fun (count, digit) ->
            match count with
            | 6 when (digit ||| decoded[4]) = digit -> decoded[9] <- digit
            | 6 when (digit ||| decoded[1]) = digit -> decoded[0] <- digit
            | 6 -> decoded[6] <- digit
            | _ -> ())
    digitsWithCounts
    |> List.iter
        (fun (count, digit) ->
            match count with
            | 5 when (digit ||| decoded[6]) = decoded[6] -> decoded[5] <- digit
            | 5 when (digit ||| decoded[9]) = decoded[9] -> decoded[3] <- digit
            | 5 -> decoded[2] <- digit
            | _ -> ())
    let mutable mapping = [| 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy ; 0uy |]
    mapping[0] <- decoded[1] ^^^ decoded[7] // compare 1 and 7 to find a
    mapping[2] <- decoded[5] ^^^ decoded[9] // compare 5 and 9 to find c
    mapping[3] <- decoded[0] ^^^ decoded[8] // compare 0 and 8 to find d
    mapping[4] <- decoded[8] ^^^ decoded[9] // compare 8 and 9 to find e
    mapping[1] <- (decoded[8] - mapping[4]) ^^^ decoded[3] // compare (8 - e) and 3 to find b
    mapping[5] <- (decoded[8] - mapping[1]) ^^^ decoded[2] // compare (8 - b) and 2 to find f
    mapping[6] <- (decoded[4] ||| mapping[0]) ^^^ decoded[9] // compare (4 + a) and 9 to find g
    getOutput output (mapping |> List.ofArray)

let decodeDisplays: (seq<byte> * seq<byte>) list -> uint32 = List.map decodeDisplay >> List.sum

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

module Tests =
    open Xunit
    open FsUnit

    [<Fact>]
    let ``Test counting of easy digits`` () =
        [ "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
          "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
          "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
          "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
          "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
          "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
          "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
          "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
          "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
          "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce" ]
        |> List.map parse |> countEasyDigits |> should equal 26

    [<Fact>]
    let ``Test decoding of all displays`` () =
        [ "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
          "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
          "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
          "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
          "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
          "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
          "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
          "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
          "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
          "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce" ]
        |> List.map parse |> decodeDisplays |> should equal 61229u

    [<Fact>]
    let ``Test decoding of individual displays`` () =
        [ "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
          "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
          "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
          "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
          "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
          "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
          "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
          "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
          "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
          "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce" ]
        |> List.map parse |> List.map decodeDisplay |> should equivalent [
            8394u; 9781u; 1197u; 9361u; 4873u; 8418u; 4548u; 1625u; 8717u; 4315u
        ]

    [<Fact>]
    let ``Test get output 123`` () =
        let output =
            [ A ||| B ||| C ||| E ||| F ||| G
              C ||| F
              A ||| C ||| D ||| E ||| G
              A ||| C ||| D ||| F ||| G ]
        let mapping = [ A ; B ; C ; D ; E ; F ; G ]
        getOutput output mapping |> should equal 123u

    [<Fact>]
    let ``Test get output 4567`` () =
        let output =
            [ B ||| C ||| D ||| F
              A ||| B ||| D ||| F ||| G
              A ||| B ||| D ||| E ||| F ||| G
              A ||| C ||| F ]
        let mapping = [ A ; B ; C ; D ; E ; F ; G ]
        getOutput output mapping |> should equal 4567u

    [<Fact>]
    let ``Test get output 8899`` () =
        let output =
            [ A ||| B ||| C ||| D ||| E ||| F ||| G
              A ||| B ||| C ||| D ||| E ||| F ||| G
              A ||| B ||| C ||| D ||| F ||| G
              A ||| B ||| C ||| D ||| F ||| G ]
        let mapping = [ A ; B ; C ; D ; E ; F ; G ]
        getOutput output mapping |> should equal 8899u

    [<Fact>]
    let ``Test get output invalid`` () =
        let output =
            [ F ||| G
              F ||| G
              F ||| G
              F ||| G ]
        let mapping = [ A ; B ; C ; D ; E ; F ; G ]
        (fun () -> getOutput output mapping |> ignore) |> should throw typeof<ArgumentException>

    [<Fact>]
    let ``Test get output with valid mapping`` () =
        let output =
            [ B ||| D ||| E ||| F ||| A
              B ||| D ||| E ||| G ||| A
              C ||| D ||| E ||| G
              B ||| C ||| E ||| G ||| A ]
        let mapping = [ B ; C ; D ; E ; F ; G ; A ]
        getOutput output mapping |> should equal 2345u

    [<Fact>]
    let ``Test get output with invalid mapping`` () =
        let output =
            [ B ||| D ||| E ||| F ||| A
              B ||| D ||| E ||| G ||| A
              C ||| D ||| E ||| G
              B ||| C ||| E ||| G ||| A ]
        let mapping = [ A ; A ; A ; A ; A ; A ; A ]
        (fun () -> getOutput output mapping |> ignore) |> should throw typeof<ArgumentException>
