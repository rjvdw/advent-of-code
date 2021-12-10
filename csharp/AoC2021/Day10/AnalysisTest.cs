using Xunit;

namespace Day10;

public class AnalysisTest
{
    [Fact]
    public void Test1()
    {
        var result = Analysis.Parse("[({(<(())[]>[[{[]{<()<>>");
        Assert.Equal(Analysis.AnalysisType.Valid, result.Type);
        Assert.Equal("}}]])})]", result.MissingCharacters);
        Assert.Equal(288957UL, result.Score());
    }

    [Fact]
    public void Test2()
    {
        var result = Analysis.Parse("[(()[<>])]({[<{<<[]>>(");
        Assert.Equal(Analysis.AnalysisType.Valid, result.Type);
        Assert.Equal(")}>]})", result.MissingCharacters);
        Assert.Equal(5566UL, result.Score());
    }

    [Fact]
    public void Test3()
    {
        var result = Analysis.Parse("{([(<{}[<>[]}>{[]{[(<()>");
        Assert.Equal(Analysis.AnalysisType.Invalid, result.Type);
        Assert.Equal('}', result.InvalidCharacter);
        Assert.Equal(1197UL, result.Score());
    }

    [Fact]
    public void Test4()
    {
        var result = Analysis.Parse("(((({<>}<{<{<>}{[]{[]{}");
        Assert.Equal(Analysis.AnalysisType.Valid, result.Type);
        Assert.Equal("}}>}>))))", result.MissingCharacters);
        Assert.Equal(1480781UL, result.Score());
    }

    [Fact]
    public void Test5()
    {
        var result = Analysis.Parse("[[<[([]))<([[{}[[()]]]");
        Assert.Equal(Analysis.AnalysisType.Invalid, result.Type);
        Assert.Equal(')', result.InvalidCharacter);
        Assert.Equal(3UL, result.Score());
    }

    [Fact]
    public void Test6()
    {
        var result = Analysis.Parse("[{[{({}]{}}([{[{{{}}([]");
        Assert.Equal(Analysis.AnalysisType.Invalid, result.Type);
        Assert.Equal(']', result.InvalidCharacter);
        Assert.Equal(57UL, result.Score());
    }

    [Fact]
    public void Test7()
    {
        var result = Analysis.Parse("{<[[]]>}<{[{[{[]{()[[[]");
        Assert.Equal(Analysis.AnalysisType.Valid, result.Type);
        Assert.Equal("]]}}]}]}>", result.MissingCharacters);
        Assert.Equal(995444UL, result.Score());
    }

    [Fact]
    public void Test8()
    {
        var result = Analysis.Parse("[<(<(<(<{}))><([]([]()");
        Assert.Equal(Analysis.AnalysisType.Invalid, result.Type);
        Assert.Equal(')', result.InvalidCharacter);
        Assert.Equal(3UL, result.Score());
    }

    [Fact]
    public void Test9()
    {
        var result = Analysis.Parse("<{([([[(<>()){}]>(<<{{");
        Assert.Equal(Analysis.AnalysisType.Invalid, result.Type);
        Assert.Equal('>', result.InvalidCharacter);
        Assert.Equal(25137UL, result.Score());
    }

    [Fact]
    public void Test10()
    {
        var result = Analysis.Parse("<{([{{}}[<[[[<>{}]]]>[]]");
        Assert.Equal(Analysis.AnalysisType.Valid, result.Type);
        Assert.Equal("])}>", result.MissingCharacters);
        Assert.Equal(294UL, result.Score());
    }
}
