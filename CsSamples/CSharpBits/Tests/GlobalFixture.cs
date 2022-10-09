using System;
using Serilog;
using Xunit;
using Xunit.Abstractions;

namespace Tests;

/// <summary>
/// Setup for serilog 
/// </summary>
public class GlobalFixture : IDisposable
{
    private bool isInit;
    private bool isDisposed;

    public GlobalFixture()
    {
        isInit = false;
        isDisposed = false;
    }

    public void Init(ITestOutputHelper outputHelper)
    {
        if (isInit) return;

        Log.Logger = new LoggerConfiguration()
                            .WriteTo.TestOutput(outputHelper)
			                .CreateLogger();
        Log.Information("Global Fixture Setup.");

        isInit = true;
    }

    public void Dispose()
    {
        if (isDisposed)
		    return;
        isDisposed = true;
    
    }
}
