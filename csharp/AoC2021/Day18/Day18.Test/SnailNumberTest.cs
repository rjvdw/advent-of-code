using System;
using System.Reflection;
using Xunit;

namespace Day18.Test;

public class SnailNumberTest
{
    private static (bool Exploded, long? Left, long? Right) Explode(SnailNumber instance)
    {
        // needed so I can use the test cases from the website, without having to make Explode public.
        const BindingFlags bf = BindingFlags.NonPublic | BindingFlags.Instance;
        var type = instance.GetType();
        var method = type.GetMethod("Explode", bf)!;

        return (ValueTuple<bool, long?, long?>)method.Invoke(instance, new object[] { 0 })!;
    }

    [Fact]
    public void TestExplode()
    {
        var sn1 = SnailNumber.Parse("[[[[[9,8],1],2],3],4]");
        Assert.Equal((true, 9, null), Explode(sn1));
        Assert.Equal("[[[[0,9],2],3],4]", sn1.ToString());

        var sn2 = SnailNumber.Parse("[7,[6,[5,[4,[3,2]]]]]");
        Assert.Equal((true, null, 2), Explode(sn2));
        Assert.Equal("[7,[6,[5,[7,0]]]]", sn2.ToString());

        var sn3 = SnailNumber.Parse("[[6,[5,[4,[3,2]]]],1]");
        Assert.Equal((true, null, null), Explode(sn3));
        Assert.Equal("[[6,[5,[7,0]]],3]", sn3.ToString());

        var sn4 = SnailNumber.Parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        Assert.Equal((true, null, null), Explode(sn4));
        Assert.Equal("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", sn4.ToString());

        var sn5 = SnailNumber.Parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        Assert.Equal((true, null, 2), Explode(sn5));
        Assert.Equal("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", sn5.ToString());

        var sn6 = SnailNumber.Parse("[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]");
        Assert.Equal((true, 1, null), Explode(sn6));
        Assert.Equal("[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]", sn6.ToString());
        Assert.Equal((true, null, null), Explode(sn6));
        Assert.Equal("[[[[3,0],[5,3]],[4,4]],[5,5]]", sn6.ToString());
    }

    [Fact]
    public void TestAdd()
    {
        var a = SnailNumber.Parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        var b = SnailNumber.Parse("[1,1]");
        Assert.Equal("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", (a + b).ToString());

        var sum = SnailNumber.Parse("[1,1]")
                  + SnailNumber.Parse("[2,2]")
                  + SnailNumber.Parse("[3,3]")
                  + SnailNumber.Parse("[4,4]");
        Assert.Equal("[[[[1,1],[2,2]],[3,3]],[4,4]]", sum.ToString());

        sum += SnailNumber.Parse("[5,5]");
        Assert.Equal("[[[[3,0],[5,3]],[4,4]],[5,5]]", sum.ToString());

        sum += SnailNumber.Parse("[6,6]");
        Assert.Equal("[[[[5,0],[7,4]],[5,5]],[6,6]]", sum.ToString());
    }

    [Fact]
    public void TestMagnitude()
    {
        Assert.Equal(143, SnailNumber.Parse("[[1,2],[[3,4],5]]").Magnitude());
        Assert.Equal(1384, SnailNumber.Parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").Magnitude());
        Assert.Equal(445, SnailNumber.Parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").Magnitude());
        Assert.Equal(791, SnailNumber.Parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").Magnitude());
        Assert.Equal(1137, SnailNumber.Parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").Magnitude());
        Assert.Equal(3488, SnailNumber.Parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").Magnitude());
    }
}
