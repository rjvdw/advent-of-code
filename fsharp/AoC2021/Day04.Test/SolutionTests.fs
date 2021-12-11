module SolutionTests

open Board
open FsUnit
open Solution
open Xunit

[<Fact>]
let ``The correct score for the winning board should be found when playing.`` () =
    let numbers =
        [ 7uy; 4uy; 9uy; 5uy; 11uy; 17uy; 23uy; 2uy; 0uy; 14uy; 21uy; 24uy; 10uy; 16uy; 13uy; 6uy; 15uy; 25uy; 12uy
          22uy; 18uy; 20uy; 8uy; 19uy; 3uy; 26uy; 1uy ]

    let boards =
        [ { Numbers =
                [ 22uy; 13uy; 17uy; 11uy; 0uy
                  8uy; 2uy; 23uy; 4uy; 24uy
                  21uy; 9uy; 14uy; 16uy; 7uy
                  6uy; 10uy; 3uy; 18uy; 5uy
                  1uy; 12uy; 20uy; 15uy; 19uy ]
            Dim = 5
            Marked = Set.empty }
          { Numbers =
                [ 3uy; 15uy; 0uy; 2uy; 22uy
                  9uy; 18uy; 13uy; 17uy; 5uy
                  19uy; 8uy; 7uy; 25uy; 23uy
                  20uy; 11uy; 10uy; 24uy; 4uy
                  14uy; 21uy; 16uy; 12uy; 6uy ]
            Dim = 5
            Marked = Set.empty }
          { Numbers =
                [ 14uy; 21uy; 17uy; 24uy; 4uy
                  10uy; 16uy; 15uy; 9uy; 19uy
                  18uy; 8uy; 23uy; 26uy; 20uy
                  22uy; 11uy; 13uy; 6uy; 5uy
                  2uy; 0uy; 12uy; 3uy; 7uy ]
            Dim = 5
            Marked = Set.empty } ]

    numbers |> play boards |> should equal (Some 4512u)

[<Fact>]
let ``The correct score for the losing board should be found when playing.`` () =
    let numbers =
        [ 7uy; 4uy; 9uy; 5uy; 11uy; 17uy; 23uy; 2uy; 0uy; 14uy; 21uy; 24uy; 10uy; 16uy; 13uy; 6uy; 15uy; 25uy; 12uy
          22uy; 18uy; 20uy; 8uy; 19uy; 3uy; 26uy; 1uy ]

    let boards =
        [ { Numbers =
                [ 22uy; 13uy; 17uy; 11uy; 0uy
                  8uy; 2uy; 23uy; 4uy; 24uy
                  21uy; 9uy; 14uy; 16uy; 7uy
                  6uy; 10uy; 3uy; 18uy; 5uy
                  1uy; 12uy; 20uy; 15uy; 19uy ]
            Dim = 5
            Marked = Set.empty }
          { Numbers =
                [ 3uy; 15uy; 0uy; 2uy; 22uy
                  9uy; 18uy; 13uy; 17uy; 5uy
                  19uy; 8uy; 7uy; 25uy; 23uy
                  20uy; 11uy; 10uy; 24uy; 4uy
                  14uy; 21uy; 16uy; 12uy; 6uy ]
            Dim = 5
            Marked = Set.empty }
          { Numbers =
                [ 14uy; 21uy; 17uy; 24uy; 4uy
                  10uy; 16uy; 15uy; 9uy; 19uy
                  18uy; 8uy; 23uy; 26uy; 20uy
                  22uy; 11uy; 13uy; 6uy; 5uy
                  2uy; 0uy; 12uy; 3uy; 7uy ]
            Dim = 5
            Marked = Set.empty } ]

    numbers |> findLosingBoard boards |> should equal (Some 1924u)
