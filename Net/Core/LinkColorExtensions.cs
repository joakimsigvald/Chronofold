using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core;

public static class LinkColorExtensions
{
    public const byte PaletteSize = 6;
    private static readonly LinkColor[][] _complements = PrecalculateComplements();

    extension(LinkColor blend)
    {
        public LinkColor[] Complements => _complements[(int)blend];
    }

    extension(LinkColor color)
    {
        public LinkColor GetOpposite()
            => color == LinkColor.Black || color == LinkColor.White
            ? color ^ LinkColor.White
            : DoInvert((ushort)color, PaletteSize / 2);
    }

    private static LinkColor[][] PrecalculateComplements()
        => [..Enumerable.Range(0, (int)Math.Pow(2, PaletteSize)).Cast<LinkColor>()
        .Select(GetComplements)];

    private static LinkColor[] GetComplements(LinkColor blend)
    {
        Span<LinkColor> results = stackalloc LinkColor[PaletteSize];
        int count = 0;
        for (var i = 0; i < PaletteSize; i++)
        {
            var color = (LinkColor)(1 << i);
            if ((blend & color) == 0)
                results[count++] = color;
        }
        return [.. results[..count]];
    }

    private static LinkColor DoInvert(ushort color, int halfCycle)
        => LinkColor.White & (LinkColor)(color << halfCycle | color >> halfCycle);
}