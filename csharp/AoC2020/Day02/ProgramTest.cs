using Xunit;

namespace Day02;

public class ProgramTest
{
    [Fact]
    public void TestIsValidPassword1()
    {
        Assert.True(PasswordEntry.Parse("1-3 a: abcde").IsValidAccordingTo1());
        Assert.False(PasswordEntry.Parse("1-3 b: cdefg").IsValidAccordingTo1());
        Assert.True(PasswordEntry.Parse("2-9 c: ccccccccc").IsValidAccordingTo1());
    }

    [Fact]
    public void TestIsValidPassword2()
    {
        Assert.True(PasswordEntry.Parse("1-3 a: abcde").IsValidAccordingTo2());
        Assert.False(PasswordEntry.Parse("1-3 b: cdefg").IsValidAccordingTo2());
        Assert.False(PasswordEntry.Parse("2-9 c: ccccccccc").IsValidAccordingTo2());
    }
}
