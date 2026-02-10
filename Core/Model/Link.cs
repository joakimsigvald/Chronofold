using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core.Model;

public record Link(int Index, Monad Left, Monad Right)
{
    public LinkColor Color { get; private set; }

    internal void Set(LinkColor color)
    {
        Color = color;
        Left.Blend(Color);
        Right.Blend(Color);
    }

    internal void Reset()
    {
        Left.Unblend(Color);
        Right.Unblend(Color);
        Color = LinkColor.None;
    }

    public Monad OtherHalf(Monad monad) => Left == monad ? Right : Left;
}