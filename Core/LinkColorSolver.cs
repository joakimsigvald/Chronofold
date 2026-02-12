using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;
namespace Applique.Chronofold.Core;

public class LinkColorSolver(Monad[] monads)
{
    private static readonly LinkColor[][] _antiSplits = PrecalculateAntiSplits();
    private static readonly LinkColor[] _palette = _antiSplits[0];

    public void ApplyColors()
    {
        Monad[] radialMonads = [.. monads.OrderBy(m => m.RadialIndex)];
        Link[] links = [.. radialMonads.SelectMany(m => m.Links).Distinct()];
        if (links.Length < LinkColorExtensions.PaletteSize)
            return;

        ColorizeCenterMonad();
        if (!Solve(LinkColorExtensions.PaletteSize))
            throw new Exception("No valid coloring found for this topology!");

        void ColorizeCenterMonad()
        {
            int i = 0;
            foreach (var color in _palette)
                links[i++].Set(color);
        }

        bool Solve(int linkIndex)
        {
            if (linkIndex >= links.Length) return true;
            var currentLink = links[linkIndex];
            foreach (var color in GetRemainingColors())
            {
                currentLink.Set(color);
                if (Solve(linkIndex + 1))
                    return true;

                currentLink.Reset();
            }
            return false;

            LinkColor[] GetRemainingColors() => GetAntiSplit(GetForbiddenBlend());

            LinkColor GetForbiddenBlend()
            {
                var forbiddenBlend = currentLink.Left.Color | currentLink.Right.Color;
                var opposingColor = currentLink.OpposingColor;
                return LinkColor.White & (forbiddenBlend | ~opposingColor);
            }
        }
    }

    private static LinkColor[] GetAntiSplit(LinkColor blend) => _antiSplits[(int)blend];

    private static LinkColor[][] PrecalculateAntiSplits()
        => [..Enumerable.Range(0, (int)Math.Pow(2, LinkColorExtensions.PaletteSize)).Cast<LinkColor>()
        .Select(c => c.AntiSplit())];
}