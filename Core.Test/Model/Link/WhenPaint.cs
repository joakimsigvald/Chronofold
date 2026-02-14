using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;
using XspecT.Assert;
using Xunit;

namespace Applique.Chronofold.Core.Test.Model.Link;

public class WhenPaint : LinkSpec<Core.Model.Link>
{
    public WhenPaint()
        => When(_ => CreateLink().Paint(The(_linkColor)));

    [Theory]
    [InlineData(LinkColor.Black)]
    [InlineData(LinkColor.Red)]
    [InlineData(LinkColor.Green)]
    [InlineData(LinkColor.Blue)]
    [InlineData(LinkColor.Cyan)]
    [InlineData(LinkColor.Magenta)]
    [InlineData(LinkColor.Yellow)]
    public void ThenLinkHasColor(LinkColor color)
        => Given(_linkColor).Is(color).Then().Result.Color.Is(The(_linkColor));

    public class GivenLeftMonad : WhenPaint
    {
        [Theory]
        [InlineData(LinkColor.Black, LinkColor.Black, LinkColor.Black)]
        [InlineData(LinkColor.Red, LinkColor.Black, LinkColor.Red)]
        [InlineData(LinkColor.Red, LinkColor.Red, LinkColor.Red)]
        [InlineData(LinkColor.Black, LinkColor.Red, LinkColor.Red)]
        [InlineData(LinkColor.Blue, LinkColor.Red, LinkColor.Blue | LinkColor.Red)]
        public void ThenMonadIsBlended(LinkColor link, LinkColor monad, LinkColor expected)
            => Given(_leftMonad).Is(CreateMonad(monad)).And(_linkColor).Is(link)
            .Then().Result.Left.Color.Is(expected);
    }

    public class GivenRightMonad : WhenPaint
    {
        [Theory]
        [InlineData(LinkColor.Black, LinkColor.Black, LinkColor.Black)]
        [InlineData(LinkColor.Red, LinkColor.Black, LinkColor.Red)]
        [InlineData(LinkColor.Red, LinkColor.Red, LinkColor.Red)]
        [InlineData(LinkColor.Black, LinkColor.Red, LinkColor.Red)]
        [InlineData(LinkColor.Blue, LinkColor.Red, LinkColor.Blue | LinkColor.Red)]
        public void ThenMonadIsBlended(LinkColor link, LinkColor monad, LinkColor expected)
            => Given(_rightMonad).Is(CreateMonad(monad)).And(_linkColor).Is(link)
            .Then().Result.Right.Color.Is(expected);
    }

    public class GivenNoLeftMonad : WhenPaint
    {
        [Theory]
        [InlineData(LinkColor.Black)]
        [InlineData(LinkColor.Red)]
        public void ThenMonadIsNotBlended(LinkColor color)
            => Given(_leftMonad).Is(Monad.NoMonad).And(_linkColor).Is(color)
            .Then().Result.Left.Color.Is(LinkColor.Black);
    }

    public class GivenNoRightMonad : WhenPaint
    {
        [Theory]
        [InlineData(LinkColor.Black)]
        [InlineData(LinkColor.Blue)]
        public void ThenMonadIsNotBlended(LinkColor color)
            => Given(_rightMonad).Is(Monad.NoMonad).And(_linkColor).Is(color)
            .Then().Result.Right.Color.Is(LinkColor.Black);
    }
}
