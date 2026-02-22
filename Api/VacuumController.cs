using Applique.Chronofold.Contract;
using Applique.Chronofold.Contract.View;
using Microsoft.AspNetCore.Mvc;

namespace Applique.Chronofold.Api;

[ApiController]
[Route("api/vacuum")]
public class VacuumController(IVacuumService service) : ControllerBase
{
    [HttpGet()]
    public Vacuum GetVacuum() => service.CreateVacuum(2);
}