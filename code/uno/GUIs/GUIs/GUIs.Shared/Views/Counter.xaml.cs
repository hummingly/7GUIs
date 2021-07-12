using GUIs.Common;
using Windows.UI.Xaml;
using Windows.UI.Xaml.Controls;

// The User Control item template is documented at https://go.microsoft.com/fwlink/?LinkId=234236

namespace GUIs.Views
{
    public sealed partial class Counter : UserControl
    {
        public Signal<int> Model = new Signal<int>(0);

        public Counter()
        {
            InitializeComponent();
        }

        public void IncrementCounter(object sender, RoutedEventArgs e)
        {
            Model.Value += 1;
        }
    }
}
