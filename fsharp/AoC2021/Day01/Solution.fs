module Solution

let countIncreases numbers =
    let _, count =
        (match numbers with
         | head :: tail ->
             List.fold
                 (fun (previous, count) current ->
                     (current,
                      if current > previous then
                          (count + 1)
                      else
                          count))
                 (head, 0)
                 tail
         | [] -> (0, 0))

    count

module Tests =
    open Xunit

    [<Fact>]
    let ``Verify that the correct number of increases is counted.`` () =
        Assert.Equal(
            7,
            countIncreases [ 199
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
