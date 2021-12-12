module Solution

let countIncreases (windowSize: int) (numbers: int seq) =
    numbers
    |> Seq.windowed windowSize
    |> Seq.map Seq.sum
    |> Seq.windowed 2
    |> Seq.filter (fun el -> el.[0] < el.[1])
    |> Seq.length
