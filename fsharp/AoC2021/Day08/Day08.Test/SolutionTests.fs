module SolutionTests

open FsUnit
open Solution
open System
open Xunit

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
    let mapping = [| A ; B ; C ; D ; E ; F ; G |]
    getOutput output mapping |> should equal 123u

[<Fact>]
let ``Test get output 4567`` () =
    let output =
        [ B ||| C ||| D ||| F
          A ||| B ||| D ||| F ||| G
          A ||| B ||| D ||| E ||| F ||| G
          A ||| C ||| F ]
    let mapping = [| A ; B ; C ; D ; E ; F ; G |]
    getOutput output mapping |> should equal 4567u

[<Fact>]
let ``Test get output 8899`` () =
    let output =
        [ A ||| B ||| C ||| D ||| E ||| F ||| G
          A ||| B ||| C ||| D ||| E ||| F ||| G
          A ||| B ||| C ||| D ||| F ||| G
          A ||| B ||| C ||| D ||| F ||| G ]
    let mapping = [| A ; B ; C ; D ; E ; F ; G |]
    getOutput output mapping |> should equal 8899u

[<Fact>]
let ``Test get output invalid`` () =
    let output =
        [ F ||| G
          F ||| G
          F ||| G
          F ||| G ]
    let mapping = [| A ; B ; C ; D ; E ; F ; G |]
    (fun () -> getOutput output mapping |> ignore) |> should throw typeof<ArgumentException>

[<Fact>]
let ``Test get output with valid mapping`` () =
    let output =
        [ B ||| D ||| E ||| F ||| A
          B ||| D ||| E ||| G ||| A
          C ||| D ||| E ||| G
          B ||| C ||| E ||| G ||| A ]
    let mapping = [| B ; C ; D ; E ; F ; G ; A |]
    getOutput output mapping |> should equal 2345u

[<Fact>]
let ``Test get output with invalid mapping`` () =
    let output =
        [ B ||| D ||| E ||| F ||| A
          B ||| D ||| E ||| G ||| A
          C ||| D ||| E ||| G
          B ||| C ||| E ||| G ||| A ]
    let mapping = [| A ; A ; A ; A ; A ; A ; A |]
    (fun () -> getOutput output mapping |> ignore) |> should throw typeof<ArgumentException>
