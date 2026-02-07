using Applique.Chronofold.Api;
using Applique.Chronofold.Contract;
using Applique.Chronofold.Core;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddRazorPages();
builder.Services.AddControllers()
    .AddApplicationPart(typeof(VacuumController).Assembly);

builder.Services.AddScoped<IVacuumService, VacuumService>();

var app = builder.Build();
app.UseHttpsRedirection();
app.UseStaticFiles();
app.UseRouting();
app.MapRazorPages();
app.MapControllers();
app.Run();