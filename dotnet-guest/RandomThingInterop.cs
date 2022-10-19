using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;
using System.Text;

namespace WasmDayDotnet;

[StructLayout(LayoutKind.Sequential)]
internal struct RandomThingRequest
{
    internal const byte RANDOM_THING_REQUEST_JOKE = 0;
    internal const byte RANDOM_THING_REQUEST_ANIMAL_FACT = 1;

    internal byte tag;
    internal RandomThingRequestVal val;
}

[StructLayout(LayoutKind.Sequential)]
internal struct RandomThingAnimalType
{
    internal const byte RANDOM_THING_ANIMAL_TYPE_CAT = 0;
    internal const byte RANDOM_THING_ANIMAL_TYPE_DOG = 1;

    internal byte animalType;
}

[StructLayout(LayoutKind.Explicit)]
internal struct RandomThingRequestVal
{
    [FieldOffset(0)]
    internal RandomThingAnimalType animalFact;
}

[StructLayout(LayoutKind.Sequential)]
internal struct RandomThingError
{
    internal const byte RANDOM_THING_ERROR_NETWORK = 0;
    internal const byte RANDOM_THING_ERROR_SERVICE = 1;
    internal const byte RANDOM_THING_ERROR_RESPONSE = 2;

    internal byte tag;
    internal RandomThingErrorVal val;
}

[StructLayout(LayoutKind.Explicit)]
internal struct RandomThingErrorVal
{
    [FieldOffset(0)]
    internal InteropString network;
    [FieldOffset(0)]
    internal UInt16 service;
}

[StructLayout(LayoutKind.Explicit)]
internal struct InteropStringOrRandomThingErrorVal
{
    [FieldOffset(0)]
    internal InteropString ok;
    [FieldOffset(0)]
    internal RandomThingError err;
}

[StructLayout(LayoutKind.Sequential)]
internal struct InteropStringOrRandomThingError
{
    internal byte is_err;
    internal InteropStringOrRandomThingErrorVal val;
}

internal static class RandomThingInterop
{
    [MethodImpl(MethodImplOptions.InternalCall)]
    internal static extern unsafe void random_thing_get_random_thing(ref RandomThingRequest req, ref InteropStringOrRandomThingError ret0);
}
