using Applique.Chronofold.Api;
using Applique.Chronofold.Contract;
using Applique.Chronofold.Core;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddControllers()
    .AddApplicationPart(typeof(VacuumController).Assembly);

builder.Services.AddScoped<IVacuumService, VacuumService>();

var app = builder.Build();
app.UseDefaultFiles();
app.UseStaticFiles();
app.MapControllers();
app.Run();