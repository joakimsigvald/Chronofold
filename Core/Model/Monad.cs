using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core.Model;

public record Monad(int LinearIndex, int RadialIndex, double X, double Y)
{
    public Link[] Links { get; internal set; } = [];
    public Monad[] Neighbours { get; internal set; } = [];
    public LinkColor Color { get; private set; }

    internal void Blend(LinkColor color) => Color |= color;
    internal void Unblend(LinkColor color) => Color &= ~color;
    internal bool IsBlended(LinkColor color) => Color.HasFlag(color);
}