namespace Applique.Chronofold.Contract;

[Flags]
public enum LinkColor
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