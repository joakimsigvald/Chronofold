using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core.Model;

public record Link(int Index, Monad Left, Monad Right)
{
    public static readonly Link NoLink = new(-1, Monad.NoMonad, Monad.NoMonad);

    public LinkColor Color { get; private set; }

    public double X => (Left.X + Right.X) / 2;
    public double Y => (Left.Y + Right.Y) / 2;

    public override string ToString() => $"{Index}: {Color} [{Left.RadialIndex}-{Right.RadialIndex}]";

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

    internal bool IsBlack => Color == LinkColor.Black;

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
        Color = LinkColor.Black;
    }

    internal LinkColor OpposingColor => OpposingLeft() & OpposingRight();

    private LinkColor OpposingLeft() => Left.Opposite(this).Color.Invert();
    private LinkColor OpposingRight() => Left.Opposite(this).Color.Invert();

    internal Monad OtherHalf(Monad monad) => Left == monad ? Right : Left;
}