using System.Runtime.InteropServices;
using System.Text;

namespace WasmDayDotnet;

/// <summary>
/// The Wasm Canonical ABI representation of a string.
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public readonly struct InteropString
{
    internal readonly nint _utf8Ptr;
    internal readonly int _utf8Length;

    internal InteropString(nint ptr, int length)
    {
        _utf8Ptr = ptr;
        _utf8Length = length;
    }

    /// <summary>
    /// Gets the string represented by the InteropString.
    /// </summary>
    public override string ToString()
        => Marshal.PtrToStringUTF8(_utf8Ptr, _utf8Length);

    /// <summary>
    /// Creates the Canonical ABI representation from a .NET string.
    /// </summary>
    public static unsafe InteropString FromString(string value)
    {
        var exactByteCount = checked(Encoding.UTF8.GetByteCount(value));
        var mem = Marshal.AllocHGlobal(exactByteCount);
        var buffer = new Span<byte>((void*)mem, exactByteCount);
        int byteCount = Encoding.UTF8.GetBytes(value, buffer);
        return new InteropString(mem, byteCount);
    }
}
