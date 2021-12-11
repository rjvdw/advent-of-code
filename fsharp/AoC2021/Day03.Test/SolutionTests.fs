module SolutionTests

open FsUnit
open Solution
open Xunit

let testData =
    [ 0b00100us
      0b11110us
      0b10110us
      0b10111us
      0b10101us
      0b01111us
      0b00111us
      0b11100us
      0b10000us
      0b11001us
      0b00010us
      0b01010us ]

[<Fact>]
let ``Test mask sequence`` () =
    maskSeq |> Seq.take 4 |> should equal [ 1us ; 2us ; 4us ; 8us ]

[<Fact>]
let ``Test bit counter`` () =
    testData |> countBits 0b10000us |> should equal (5, 7)
    testData |> countBits 0b01000us |> should equal (7, 5)
    testData |> countBits 0b00100us |> should equal (4, 8)
    testData |> countBits 0b00010us |> should equal (5, 7)
    testData |> countBits 0b00001us |> should equal (7, 5)

[<Fact>]
let ``Test power consumption`` () =
    testData |> computePowerConsumption 5 |> should equal 198u

[<Fact>]
let ``Test life support rating`` () =
    testData |> computeLifeSupportRating 5 |> should equal 230u
