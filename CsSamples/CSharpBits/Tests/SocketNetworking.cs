using Serilog;
using Serilog.Sinks.XUnit;
using Xunit;
using Xunit.Abstractions;
using System.Net.Sockets;
using System.Net.Security;
using System.Security.Cryptography.X509Certificates;
using System.Threading;
using System.Net;
using System.Text;

namespace Tests;

public class SocketNetworking : IClassFixture<GlobalFixture>
{
    public SocketNetworking(GlobalFixture fixture, ITestOutputHelper output)
    {
        fixture.Init(output);
    }

    [Fact]
    public void simple_tcp_client()
    {
        string message = "Hello World\n";
        byte[] messageBytes = System.Text.Encoding.ASCII.GetBytes(message);

        using (var client = new TcpClient("45.79.112.203", 4242))
        using (NetworkStream stream = client.GetStream())
        {

            stream.Write(messageBytes, 0, messageBytes.Length);

            var buffer = new byte[256];

            Int32 responseByteCount = stream.Read(buffer);

            // tcpbin is an echo server. It's response should match what we sent.
            Assert.Equal(messageBytes.Length, responseByteCount);

            string response = System.Text.Encoding.ASCII.GetString(buffer, 0, responseByteCount);

            Assert.Equal(message, response);
        }
    }

    [Fact]
    public void simple_tcp_client_tls()
    {
        string message = "Hello World\n";
        byte[] messageBytes = System.Text.Encoding.ASCII.GetBytes(message);

        // Must use server name, not IP, with TLS. Otherwise it will not match the certificate.
        string serverName = "tcpbin.com";

        bool ValidateServerCertificate(object sender, X509Certificate certificate, X509Chain chain, SslPolicyErrors sslPolicyErrors)
        {
            if (sslPolicyErrors == SslPolicyErrors.None)
                return true;

            Assert.True(false, "Expected valid certificate.");
            return false;
        }

        using (var client = new TcpClient(serverName, 4243))
        using (SslStream sslStream = new SslStream(client.GetStream(), false, new RemoteCertificateValidationCallback(ValidateServerCertificate), null))
        {
            sslStream.AuthenticateAsClient(serverName);
            sslStream.Write(messageBytes, 0, messageBytes.Length);

            var buffer = new byte[256];

            Int32 responseByteCount = sslStream.Read(buffer);

            // tcpbin is an echo server. It's response should match what we sent.
            Assert.Equal(messageBytes.Length, responseByteCount);

            string response = System.Text.Encoding.ASCII.GetString(buffer, 0, responseByteCount);

            Assert.Equal(message, response);
        }
    }

    [Fact]
    public void simple_tcp_server()
    {
        var cancelSrc = new CancellationTokenSource();
        var cancel = cancelSrc.Token;

        // start echo server 
        var thread = new Thread(() =>
        {
            var listener = new TcpListener(IPAddress.Any, 9999);
            listener.Start();
            using (TcpClient client = listener.AcceptTcpClient())  //if a connection exists, the server will accept it
            {
                NetworkStream ns = client.GetStream(); //networkstream is used to send/receive messages
                cancel.Register(delegate { ns.Close(); });
                while (client.Connected && !cancel.IsCancellationRequested)  //while the client is connected, we look for incoming messages
                {
                    byte[] msg = new byte[1024];     //the messages arrive as byte array
                    try
                    {
                        Int32 len = ns.Read(msg, 0, msg.Length);   //the same networkstream reads the message sent by the client
                        ns.Write(msg, 0, len);
                    }
                    catch (IOException)
                    {
                        throw;
                    }
                }
            }
        });
        thread.Start();

        Thread.Sleep(2000);

        string message = "Hello World\n";
        byte[] messageBytes = System.Text.Encoding.ASCII.GetBytes(message);

        using (var client = new TcpClient("127.0.0.1", 9999))
        {
            Assert.True(client.Connected);
            using (NetworkStream stream = client.GetStream())
            {

                stream.Write(messageBytes, 0, messageBytes.Length);

                var buffer = new byte[256];

                Int32 responseByteCount = stream.Read(buffer);

                Assert.Equal(messageBytes.Length, responseByteCount);

                string response = System.Text.Encoding.ASCII.GetString(buffer, 0, responseByteCount);

                Assert.Equal(message, response);
            }
        }
        cancelSrc.Cancel();
        thread.Join();
    }

    [Fact]
    public async void multiclient_tcp_server()
    {
        return; // currently broken.

        var cancelSrc = new CancellationTokenSource();
        var cancel = cancelSrc.Token;

        // start echo server 
        var thread = new Thread(() =>
        {
            var listener = new TcpListener(IPAddress.Any, 8888);
            cancel.Register(() => { listener.Stop(); });
            listener.Start();

            while (!cancel.IsCancellationRequested)
            {
                try
                {
                    TcpClient client = listener.AcceptTcpClient();

                    Task.Run(() =>
                    {
                        NetworkStream ns = client.GetStream(); //networkstream is used to send/receive messages

                        cancel.Register(() => { ns.Close(); });
                        while (client.Connected && cancel.IsCancellationRequested)  //while the client is connected, we look for incoming messages
                        {
                            byte[] msg = new byte[1024];     //the messages arrive as byte array
                            try
                            {
                                Int32 len = ns.Read(msg, 0, msg.Length);   //the same networkstream reads the message sent by the client
                                ns.Write(msg, 0, len);
                            }
                            catch (IOException)
                            {
                                throw;
                            }
                        }
                        
                    });
                }
                catch (SocketException)
                {
                    throw;
                }
            }
        });

        thread.Start();

        Thread.Sleep(2000);

        async Task<string> connectTcpClient(string message)
        {
            byte[] messageBytes = System.Text.Encoding.ASCII.GetBytes(message);

            string response;
            using (var client = new TcpClient("127.0.0.1", 8888))
            using (NetworkStream stream = client.GetStream())
            {

                stream.Write(messageBytes, 0, messageBytes.Length);

                var buffer = new byte[256];

                Int32 responseByteCount = await stream.ReadAsync(buffer);

                Assert.Equal(messageBytes.Length, responseByteCount);

                response = System.Text.Encoding.ASCII.GetString(buffer, 0, responseByteCount);

                Assert.Equal(message, response);
            }
            return response;
        }

        var m1 = connectTcpClient("Message One");
        var m2 = connectTcpClient("Message Two");

        Assert.Equal(await m1, "Message One");
        Assert.Equal(await m2, "Message Two");

        cancelSrc.Cancel();
        thread.Join();
    }
}