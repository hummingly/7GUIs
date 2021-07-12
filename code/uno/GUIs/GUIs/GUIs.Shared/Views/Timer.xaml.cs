using GUIs.Common;
using System;
using System.Diagnostics;
using Windows.UI.Xaml;
using Windows.UI.Xaml.Controls;
using Windows.UI.Xaml.Controls.Primitives;
using Windows.UI.Xaml.Data;

// The User Control item template is documented at https://go.microsoft.com/fwlink/?LinkId=234236

namespace GUIs.Views
{
    public sealed partial class Timer : UserControl
    {
        private Stopwatch stopWatch = new Stopwatch();
        private DispatcherTimer timer = new DispatcherTimer();
        public Signal<int> ElapsedTime = new Signal<int>(0);
        public Signal<int> Duration = new Signal<int>(15000);
        private long _elapsedTime = 0;

        public Timer()
        {
            InitializeComponent();
            timer.Interval = TimeSpan.FromMilliseconds(100);
            timer.Tick += HandleTick;
            StartTimer();
        }

        public void ResetTimer(object sender, RoutedEventArgs e)
        {
            ElapsedTime.Value = 0;
            StartTimer();
        }

        public void DurationChanged(object sender, RangeBaseValueChangedEventArgs e)
        {
            ElapsedTime.Value = Math.Min(ElapsedTime.Value, Duration.Value);
            StartTimer();
        }

        public string FormatElapsedTime(int milliseconds)
        {
            double seconds = milliseconds / 1000.0;
            return string.Format("Elapsed Time: {0:0.0}s", seconds);
        }

        public string FormatDuration(int milliseconds)
        {
            double seconds = milliseconds / 1000.0;
            return string.Format("Duration: {0:0.0}s", seconds);
        }

        private void HandleTick(object sender, object e)
        {
            var currentTime = stopWatch.ElapsedMilliseconds;
            var span = currentTime - _elapsedTime;
            ElapsedTime.Value += (int)span;
            if (ElapsedTime.Value < Duration.Value)
            {
                _elapsedTime = currentTime;
            }
            else
            {
                ElapsedTime.Value = Duration.Value;
                timer.Stop();
                stopWatch.Reset();
            }
        }


        private void StartTimer()
        {
            if (!timer.IsEnabled && ElapsedTime.Value < Duration.Value)
            {
                _elapsedTime = 0;
                stopWatch.Reset();
                stopWatch.Start();
                timer.Start();
            }

        }
    }


    public class MillisecondToDouble : IValueConverter
    {
        public object Convert(object value, Type targetType, object parameter, string language)
        {
            double v = (int)value;
            return Math.Round(v / 1000.0, 1);
        }

        public object ConvertBack(object value, Type targetType, object parameter, string language)
        {
            double v = Math.Round(((double)value) * 1000.0);
            return (int)v;
        }
    }
}
