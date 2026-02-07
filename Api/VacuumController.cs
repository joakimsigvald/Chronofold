using Applique.Chronofold.Contract;
using Microsoft.AspNetCore.Mvc;

namespace Applique.Chronofold.Api;

[ApiController]
[Route("api/vacuum")]
public class VacuumController : ControllerBase
{
    [HttpGet("version")]
    public string GetVersion() => "Chronofold API v0.0.1";

    [HttpGet("monads")]
    public VacuumState GetMonads() => new([new Monad(1, 0, 0)]);
}