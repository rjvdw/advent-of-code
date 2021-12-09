module Solution

let solve = "Hello, World!"

module Tests =
    open FsUnit
    open Xunit

    [<Fact>]
    let ``Test 1`` () =
        0 |> should equal 0
