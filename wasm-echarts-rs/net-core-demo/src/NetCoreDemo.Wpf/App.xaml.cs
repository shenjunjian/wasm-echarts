using System.Windows;

namespace NetCoreDemo.Wpf;

public partial class App : Application
{
    protected override void OnStartup(StartupEventArgs e)
    {
        if (e.Args.Any(arg => string.Equals(arg, "--smoke", StringComparison.OrdinalIgnoreCase)))
        {
            try
            {
                ShutdownMode = ShutdownMode.OnExplicitShutdown;
                Environment.ExitCode = GallerySmoke.Run();
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine(ex);
                Environment.ExitCode = 1;
            }

            Shutdown();
            return;
        }

        base.OnStartup(e);
    }
}
