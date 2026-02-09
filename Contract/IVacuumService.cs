using Applique.Chronofold.Contract.View;

namespace Applique.Chronofold.Contract;

public interface IVacuumService
{
    Vacuum CreateVacuum(int depth);
}