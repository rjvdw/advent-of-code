﻿// For more information see https://aka.ms/fsharp-console-apps
printfn "Hello from F#"

module Tests =
    open Xunit

    [<Fact>]
    let ``Test 1`` () =
        Assert.Equal(0, 0)
