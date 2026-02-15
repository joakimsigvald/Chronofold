using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;
namespace Applique.Chronofold.Core;

public class LinkColorSolver(Monad[] monads)
{
    public void Solve()
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
            foreach (var color in LinkColor.Black.Complements)
                links[i++].Paint(color);
        }

        bool Solve(int linkIndex)
        {
            if (linkIndex >= links.Length) 
                return true;

            var currentLink = links[linkIndex];
            foreach (var color in currentLink.AvailableColors)
            {
                currentLink.Paint(color);
                if (Solve(linkIndex + 1))
                    return true;

                currentLink.Unpaint();
            }
            return false;
        }
    }
}