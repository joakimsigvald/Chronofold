namespace Applique.Chronofold.Contract;

public interface IVacuumService
{
    IEnumerable<Monad> CreateMonads();
}