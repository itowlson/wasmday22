namespace WasmDayDotnet;

public static class App
{
    [ConsoleHandler]
    public static InteropString HandleConsoleInput(InteropString input)
    {
        var inputStr = input.ToString();
        var response = $"{input} {input}";
        return InteropString.FromString(response);
    }
}
