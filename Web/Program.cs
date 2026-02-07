using Applique.Chronofold.Api;

var builder = WebApplication.CreateBuilder(args);

// 1. Add Razor Pages for the UI
builder.Services.AddRazorPages();

// 2. Add Controllers and point them to your Api project assembly
builder.Services.AddControllers()
    .AddApplicationPart(typeof(VacuumController).Assembly);

var app = builder.Build();

// 3. Simple Pipeline
app.UseHttpsRedirection();
app.UseStaticFiles(); // Essential for d3.min.js in /lib
app.UseRouting();

// 4. Map the World
app.MapRazorPages();
app.MapControllers();

app.Run();