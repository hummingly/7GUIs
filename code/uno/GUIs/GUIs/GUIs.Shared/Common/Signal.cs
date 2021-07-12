using System.ComponentModel;

namespace GUIs.Common
{
    public class Signal<T> : INotifyPropertyChanged
    {
        private T value;

        public Signal(T initialValue)
        {
            value = initialValue;
        }

        public T Value { get => value; set { this.value = value; PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(null)); } }

        public event PropertyChangedEventHandler PropertyChanged;
    }
}
