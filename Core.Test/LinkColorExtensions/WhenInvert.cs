using Applique.Chronofold.Contract;
using XspecT;
using XspecT.Assert;
using Xunit;

namespace Applique.Chronofold.Core.Test.LinkColorExtensions;

public class WhenInvert : Spec
{
    [Theory]
    [InlineData(LinkColor.Black, LinkColor.White)]
    [InlineData(LinkColor.Red, LinkColor.Cyan)]
    [InlineData(LinkColor.Green, LinkColor.Magenta)]
    [InlineData(LinkColor.Blue, LinkColor.Yellow)]
    [InlineData(LinkColor.Cyan, LinkColor.Red)]
    [InlineData(LinkColor.Magenta, LinkColor.Green)]
    [InlineData(LinkColor.Yellow, LinkColor.Blue)]
    [InlineData(LinkColor.White, LinkColor.Black)]
    public void ThenInvert(LinkColor from, LinkColor to) => from.Invert().Is(to);
}