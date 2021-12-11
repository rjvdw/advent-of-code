module BoardTests

open Board
open FsUnit
open Xunit

[<Fact>]
let ``Parsing of a squid bingo board should return a valid board`` () =
    let lines =
        [ "22 13 17 11  0"
          " 8  2 23  4 24"
          "21  9 14 16  7"
          " 6 10  3 18  5"
          " 1 12 20 15 19" ]

    let expected =
        { Numbers =
              [ 22uy; 13uy; 17uy; 11uy; 0uy
                8uy; 2uy; 23uy; 4uy; 24uy
                21uy; 9uy; 14uy; 16uy; 7uy
                6uy; 10uy; 3uy; 18uy; 5uy
                1uy; 12uy; 20uy; 15uy; 19uy ]
          Dim = 5
          Marked = Set.empty }

    lines |> parseBoard |> should equal expected

[<Fact>]
let ``The correct position should be marked when marking a number`` () =
    let board =
        { Numbers =
              [ 22uy; 13uy; 17uy; 11uy; 0uy
                8uy; 2uy; 23uy; 4uy; 24uy
                21uy; 9uy; 14uy; 16uy; 7uy
                6uy; 10uy; 3uy; 18uy; 5uy
                1uy; 12uy; 20uy; 15uy; 19uy ]
          Dim = 5
          Marked = Set.empty }

    (mark 23uy board).Marked
    |> should equivalent (set [ 7 ])

[<Fact>]
let ``A board without any numbers marked should not be marked as bingo`` () =
    let board =
        { Numbers =
              [ 22uy; 13uy; 17uy; 11uy; 0uy
                8uy; 2uy; 23uy; 4uy; 24uy
                21uy; 9uy; 14uy; 16uy; 7uy
                6uy; 10uy; 3uy; 18uy; 5uy
                1uy; 12uy; 20uy; 15uy; 19uy ]
          Dim = 5
          Marked = Set.empty }

    board |> bingo |> should be False

[<Fact>]
let ``A board with some numbers marked that do not form a row or column should not be marked as bingo`` () =
    let board =
        { Numbers =
              [ 22uy; 13uy; 17uy; 11uy; 0uy
                8uy; 2uy; 23uy; 4uy; 24uy
                21uy; 9uy; 14uy; 16uy; 7uy
                6uy; 10uy; 3uy; 18uy; 5uy
                1uy; 12uy; 20uy; 15uy; 19uy ]
          Dim = 5
          Marked = set [ 1; 2; 5; 8; 11; 16; 21 ] }

    board |> bingo |> should be False

[<Fact>]
let ``A board with a full column should be marked as bingo`` () =
    let board =
        { Numbers =
              [ 22uy; 13uy; 17uy; 11uy; 0uy
                8uy; 2uy; 23uy; 4uy; 24uy
                21uy; 9uy; 14uy; 16uy; 7uy
                6uy; 10uy; 3uy; 18uy; 5uy
                1uy; 12uy; 20uy; 15uy; 19uy ]
          Dim = 5
          Marked = set [ 1; 6; 11; 16; 21 ] }

    board |> bingo |> should be True

[<Fact>]
let ``A board with a full row should be marked as bingo`` () =
    let board =
        { Numbers =
              [ 22uy; 13uy; 17uy; 11uy; 0uy
                8uy; 2uy; 23uy; 4uy; 24uy
                21uy; 9uy; 14uy; 16uy; 7uy
                6uy; 10uy; 3uy; 18uy; 5uy
                1uy; 12uy; 20uy; 15uy; 19uy ]
          Dim = 5
          Marked = set [ 5; 6; 7; 8; 9 ] }

    board |> bingo |> should be True

[<Fact>]
let ``The correct score should be computed for a board`` () =
    let board =
        { Numbers =
              [ 14uy; 21uy; 17uy; 24uy; 4uy
                10uy; 16uy; 15uy; 9uy; 19uy
                18uy; 8uy; 23uy; 26uy; 20uy
                22uy; 11uy; 13uy; 6uy; 5uy
                2uy; 0uy; 12uy; 3uy; 7uy ]
          Dim = 5
          Marked =
              set [ 0; 1; 2; 3; 4; 8; 12; 16; 19; 20; 21; 24 ] }

    board |> score 24uy |> should equal 4512u
