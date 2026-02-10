using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;
namespace Applique.Chronofold.Core;

public class VacuumSolver(Monad[] monads, Link[] links)
{
    public void ApplyNeighbours()
    {
        Dictionary<int, Link[]> neighbours = GetNeighbours();
        foreach (var monad in monads)
        {
            monad.Links = neighbours.TryGetValue(monad.LinearIndex, out var val)
                ? [.. val.OrderBy(l => l.Index)]
                : [];
            monad.Neighbours = [.. monad.Links.Select(l => l.OtherHalf(monad))];
        }
    }

    public void ApplyColors()
    {
        var colorPalette = Enum.GetValues<LinkColor>()[1..];
        if (!Solve(0, colorPalette))
            throw new Exception("No valid coloring found for this topology!");
    }

    private bool Solve(int linkIndex, LinkColor[] palette)
    {
        if (linkIndex >= links.Length) return true;
        var currentLink = links[linkIndex];
        foreach (var color in palette.Where(IsColorValid))
        {
            currentLink.Set(color);
            if (Solve(linkIndex + 1, palette))
                return true;

            currentLink.Reset();
        }
        return false;

        bool IsColorValid(LinkColor color) 
            => !currentLink.Left.IsBlended(color) && !currentLink.Right.IsBlended(color);
    }

    Dictionary<int, Link[]> GetNeighbours()
        => links
        .Select(l => (m: l.Left, l))
        .Concat(links.Select(l => (m: l.Right, l)))
        .GroupBy(p => p.m.LinearIndex)
        .ToDictionary(g => g.Key, g => g.Select(g => g.l).ToArray());
}