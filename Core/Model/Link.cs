using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core.Model;

public record Link(int Index, Monad Left, Monad Right)
{
    public LinkColor Color { get; internal set; }
    public Monad OtherHalf(Monad monad) => Left == monad ? Right : Left;
}