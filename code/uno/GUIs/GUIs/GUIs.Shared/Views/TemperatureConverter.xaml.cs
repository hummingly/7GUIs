using GUIs.Common;
using System;
using System.Globalization;
using System.Linq;
using Windows.UI.Xaml.Controls;

// The User Control item template is documented at https://go.microsoft.com/fwlink/?LinkId=234236

namespace GUIs.Views
{
    public sealed partial class TemperatureConverter : UserControl
    {
        private Signal<string> Celsius = new Signal<string>("");
        private Signal<string> Fahrenheit = new Signal<string>("");

        public TemperatureConverter()
        {
            InitializeComponent();
        }

        public void FilterNumbers(TextBox sender, TextBoxBeforeTextChangingEventArgs e)
        {
            e.Cancel = e.NewText.Any(c => !char.IsDigit(c));
        }

        public void ChangedCelsius(TextBox sender, TextBoxTextChangingEventArgs e)
        {
            var text = sender.Text;
            if (text.Length > 0)
            {
                var celsius = double.Parse(text, NumberStyles.AllowLeadingSign);
                Fahrenheit.Value = Math.Round(celsius * 1.8 + 32.0).ToString();
            }
        }

        public void ChangedFahrenheit(TextBox sender, TextBoxTextChangingEventArgs e)
        {
            var text = sender.Text;
            if (text.Length > 0)
            {
                var fahrenheit = double.Parse(text, NumberStyles.AllowLeadingSign);
                Celsius.Value = Math.Round((fahrenheit - 32.0) / 1.8).ToString();
            }
        }
    }
}
