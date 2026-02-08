namespace Applique.Chronofold.Contract;

[Flags] public enum LinkColor 
{
    Red = 1,
    Green = 1 << 1,
    Blue = 1 << 2,
    Cyan = 1 << 3,
    Magenta = 1 << 4,
    Yellow = 1 << 5
}