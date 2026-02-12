using Applique.Chronofold.Core.Model;
namespace Applique.Chronofold.Core;

public static class LinkGenerator
{
    public static Link[] CreateAllLinks(int depth, Coordinate[] coordinates, Monad[] monads)
    {
        int count = 0;
        return [.. coordinates.SelectMany(CreateLinks)];

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
}