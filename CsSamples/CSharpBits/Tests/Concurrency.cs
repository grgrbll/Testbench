using System;
using System.Threading;
using System.Collections.Concurrent;
using Xunit;
using Xunit.Abstractions;

namespace Tests
{
	public class Concurrency : IClassFixture<GlobalFixture>
	{
		public Concurrency(GlobalFixture fixture, ITestOutputHelper output)
		{
			fixture.Init(output);
		}

        [Fact]
		public void shared_queue_with_blocking_collection()
		{
			var messageQueueIn = new BlockingCollection<string>();
			var messageQueueOut = new BlockingCollection<string>();

			var cancelSource = new CancellationTokenSource();
			var cancel = cancelSource.Token;

			var processingTask = Task.Run(() => {
				try
				{
					while (!cancel.IsCancellationRequested)
					{
						string input = messageQueueIn.Take(cancel);
						string output = new String(input.Reverse().ToArray());
						messageQueueOut.Add(output);
					}
				}
				catch (OperationCanceledException)
				{
					return;
				}
				return;
			});

			messageQueueIn.Add("12345");
			Assert.Equal("54321", messageQueueOut.Take());

			// processing thread completes after cancelation is called.
			cancelSource.Cancel();
			Assert.True(processingTask.Wait(1000));
		}

        [Fact]
		public async void thread_pool()
		{
			// Use ThreadPool when you need to executed many CPU bound
			// operations on a limited number of thread.
			// For IO bound operations, prefer Tasks.
			var promise = new TaskCompletionSource<string>();
			void workItem(object? state)
			{
				Assert.True(state != null);

                string? input = (string?)state?.GetType().GetProperty("Input")?.GetValue(state, null);
                Assert.Equal("12345", input);

                var promise = (TaskCompletionSource<string>?)state?.GetType().GetProperty("Promise")?.GetValue(state, null);
				promise?.SetResult(new String(input?.Reverse().ToArray()));

				return;
			}

			var callback = new WaitCallback(workItem);
			ThreadPool.QueueUserWorkItem(callback, new { Input = "12345", Promise = promise });
		    Assert.Equal("54321", await promise.Task);
		}
	}
}

