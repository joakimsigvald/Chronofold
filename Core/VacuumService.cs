using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;

namespace Applique.Chronofold.Core;

public class VacuumService : IVacuumService
{
    public Contract.View.Vacuum CreateVacuum(int depth)
    {
        var vacuum = Vacuum.Create(depth);
        return new([.. vacuum.Monads.Select(ToView)], [.. vacuum.Links.Select(ToView)], LinkColorExtensions.PaletteSize);
    }

    private static Contract.View.Monad ToView(Monad monad)
        => new(monad.Id, 
            monad.X, 
            monad.Y, 
            [.. monad.Links.Select(l => l.Id)], 
            monad.Sequence,
            [..monad.Sequence.Select(_ => false)],
            monad.RadialIndex % LinkColorExtensions.PaletteSize);

    private static Contract.View.Link ToView(Link link) 
        => new(link.Id, link.X, link.Y, link.ColorName, link.Left.Id, link.Right.Id);
}