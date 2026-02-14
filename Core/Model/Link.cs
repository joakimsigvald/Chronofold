using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core.Model;

public class Link(int index, Monad left, Monad right)
{
    public static readonly Link NoLink = new(-1, Monad.NoMonad, Monad.NoMonad);

    public LinkColor Color { get; private set; }

    public int Index => index;
    public Monad Left => left;
    public Monad Right => right;

    public double X => (Left.X + Right.X) / 2;
    public double Y => (Left.Y + Right.Y) / 2;

    public override string ToString() => $"[{Index}: {Color}, {Left}-{Right}]";

    public static IEnumerable<Link> Generate(int depth, Coordinate[] coordinates, Monad[] monads)
    {
        int count = 0;
        return coordinates.SelectMany(CreateLinks);

        IEnumerable<Link> CreateLinks(Coordinate c, int index)
        {
            if (c.Col < c.Width - 1)
                yield return new(count++, monads[index], monads[index + 1]);
            if (c.Row == depth)
                yield break;

            var offset = c.Row < 0 ? c.Width : c.Width - 1;
            if (c.Col < c.Width - 1 || c.Row < 0)
                yield return new(count++, monads[index], monads[index + offset + 1]);
            if (c.Col > 0 || c.Row < 0)
                yield return new(count++, monads[index], monads[index + offset]);
        }
    }

    public Link Paint(LinkColor color)
    {
        Color = color;
        if (Left != Monad.NoMonad)
            Left.Blend(Color);
        if (Right != Monad.NoMonad)
            Right.Blend(Color);
        return this;
    }

    public Link Unpaint()
    {
        Left.Unblend(Color);
        Right.Unblend(Color);
        Color = LinkColor.Black;
        return this;
    }

    /// <summary>
    /// All colors used by other links to either left or rights monad are forbidden for this link.
    /// In addition, if the opposite link of either left or right is colored, 
    /// then this link is only allowed to have the inverse of that color (= opposing color)
    /// </summary>
    public LinkColor ForbiddenBlend => LinkColor.White & (Left.Color | Right.Color | ~OpposingColor);

    internal Monad OtherHalf(Monad monad) => Left == monad ? Right : Left;

    private LinkColor OpposingColor => OpposingLeft() & OpposingRight();
    private LinkColor OpposingLeft() => Left.Opposite(this).Color.Invert();
    private LinkColor OpposingRight() => Right.Opposite(this).Color.Invert();
}