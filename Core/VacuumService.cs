using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;

namespace Applique.Chronofold.Core;

public class VacuumService : IVacuumService
{
    public Contract.View.Vacuum CreateVacuum(int depth)
    {
        var vacuum = Vacuum.Create(depth);
        return new([.. vacuum.Monads.Select(ToView)], [.. vacuum.Links.Select(ToView)]);
    }

    private static Contract.View.Monad ToView(Monad monad)
    {
        return new(GetId(monad), monad.X, monad.Y, [.. monad.Links.Select(l => l.Id)], monad.Sequence);
    }

    private static Contract.View.Link ToView(Link link) 
        => new(GetId(link), link.X, link.Y, GetColor(link), ToView(link.Left), ToView(link.Right));

    public static string GetId(Monad monad) => $"{monad.RadialIndex + 1}";
    public static string GetId(Link link) => $"{link.Index + 1}";
    public static string GetColor(Link link) => $"{link.Color}".ToLower();
}