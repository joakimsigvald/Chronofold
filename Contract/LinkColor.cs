namespace Applique.Chronofold.Contract;

[Flags] public enum LinkColor 
{
    Black = 0,
    Red = 1,
    Green = 1 << 1,
    Blue = 1 << 2,
    Cyan = 1 << 3,
    Magenta = 1 << 4,
    Yellow = 1 << 5,
    White = (1 << 6) - 1,
}

public static class LinkColorExtensions 
{
    public const byte PaletteSize = 6;

    public static LinkColor[] AntiSplit(this LinkColor blend)
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

    public static LinkColor Invert(this LinkColor color) 
        => color switch
        {
            LinkColor.Black => LinkColor.White,
            LinkColor.White => LinkColor.Black,
            _ => (LinkColor)(((int)color + 3) % 6)
        };
}