using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core;

public class VacuumService : IVacuumService
{
    public Contract.View.Vacuum CreateVacuum(int depth)
    {
        var vacuum = VacuumGenerator.CreateVacuum(depth);
        return new([.. vacuum.Monads.Select(ToView)], [.. vacuum.Links.Select(ToView)]);
    }

    private static Contract.View.Monad ToView(Model.Monad monad) => new(GetId(monad), monad.X, monad.Y);

    private static Contract.View.Link ToView(Model.Link link) => new(GetId(link), link.X, link.Y, GetColor(link));

    public static string GetId(Model.Monad monad) => $"{monad.RadialIndex + 1}";
    public static string GetId(Model.Link link) => $"{link.Index + 1}";
    public static string GetColor(Model.Link link) 
        => link.Color switch 
        {
            LinkColor.Red => "red",
            LinkColor.Green => "green",
            LinkColor.Blue => "blue",
            LinkColor.Cyan => "cyan",
            LinkColor.Magenta => "magenta",
            LinkColor.Yellow=> "yellow",
            _ => throw new NotImplementedException($"{link.Color}")
        };
}