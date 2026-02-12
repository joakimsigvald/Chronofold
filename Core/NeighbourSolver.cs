using Applique.Chronofold.Core.Model;
namespace Applique.Chronofold.Core;

public class NeighbourSolver(Monad[] monads, Link[] links)
{
    public void ApplyNeighbours()
    {
        Dictionary<int, Link[]> neighbours = GetNeighbours();
        foreach (var monad in monads)
        {
            monad.Links = neighbours.TryGetValue(monad.LinearIndex, out var links)
                ? [.. SortClockwise(links)]
                : [];
            monad.Neighbours = [.. monad.Links.Select(l => l.OtherHalf(monad))];
        }
    }

    private static IEnumerable<Link> SortClockwise(Link[] linearLinks)
        => linearLinks[3..].Prepend(linearLinks[1]).Append(linearLinks[2]).Append(linearLinks[0]);

    Dictionary<int, Link[]> GetNeighbours()
        => links
        .Select(l => (m: l.Left, l))
        .Concat(links.Select(l => (m: l.Right, l)))
        .GroupBy(p => p.m.LinearIndex)
        .ToDictionary(g => g.Key, g => g.Select(g => g.l).OrderBy(l => l.Index).ToArray());
}