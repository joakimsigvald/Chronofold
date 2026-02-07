using Applique.Chronofold.Contract;
using Microsoft.AspNetCore.Mvc;

namespace Applique.Chronofold.Api;

[ApiController]
[Route("api/vacuum")]
public class VacuumController(IVacuumService service) : ControllerBase
{
    [HttpGet("version")]
    public string GetVersion() => "Chronofold API v0.0.1";

    [HttpGet()]
    public VacuumState GetVacuum() => service.CreateVacuum();
}