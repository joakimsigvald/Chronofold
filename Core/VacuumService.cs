using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core;

public class VacuumService : IVacuumService
{
    public Monad[] GetMonads() => [new Monad(1, -1, 0), new Monad(2, 1, 0)];
}