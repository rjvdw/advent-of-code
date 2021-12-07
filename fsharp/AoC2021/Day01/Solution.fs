module Solution

let countIncreases windowSize =
    Seq.windowed windowSize
    >> Seq.map Seq.sum
    >> Seq.windowed 2
    >> Seq.filter (fun el -> el[0] < el[1])
    >> Seq.length

module Tests =
    open Xunit

    [<Fact>]
    let ``Verify that the correct number of increases is counted when the window size is 1.`` () =
        Assert.Equal(
            7,
            countIncreases
                1
                [ 199
                  200
                  208
                  210
                  200
                  207
                  240
                  269
                  260
                  263 ]
        )

    [<Fact>]
    let ``Verify that the correct number of increases is counted when the window size is 3.`` () =
        Assert.Equal(
            5,
            countIncreases
                3
                [ 199
                  200
                  208
                  210
                  200
                  207
                  240
                  269
                  260
                  263 ]
        )
