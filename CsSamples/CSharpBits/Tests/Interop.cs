using System;
using Serilog;
using Xunit;
using Xunit.Abstractions;
using System.Runtime.InteropServices;

namespace Tests
{
    public class Interop : IClassFixture<GlobalFixture>
    {
		public Interop(GlobalFixture fixture, ITestOutputHelper output)
		{
			fixture.Init(output);
		}

        [DllImport("libSystem")]
        private static extern int getpid();

        [Fact]
        void pinvoke()
        {
            Log.Information("Process ID: {pid}", getpid());
	    }
    }
}

