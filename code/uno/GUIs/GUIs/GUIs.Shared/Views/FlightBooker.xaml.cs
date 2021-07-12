using GUIs.Common;
using System;
using System.Collections.Generic;
using System.Linq;
using Windows.UI.Xaml;
using Windows.UI.Xaml.Controls;
using Windows.UI.Xaml.Data;
using Windows.Globalization.DateTimeFormatting;

// The User Control item template is documented at https://go.microsoft.com/fwlink/?LinkId=234236

namespace GUIs.Views
{
    public sealed partial class FlightBooker : UserControl
    {
        public Signal<FlightType> Type = new Signal<FlightType>(FlightType.OneWay);
        public Signal<DateTimeOffset> DepartureDate = new Signal<DateTimeOffset>(DateTimeOffset.Now);
        public Signal<DateTimeOffset> ReturnDate = new Signal<DateTimeOffset>(DateTimeOffset.Now);
        public static string LocaleDateFormatter => DateTimeFormatter.LongDate.Template;

        public FlightBooker()
        {
            InitializeComponent();
        }

        public static IEnumerable<string> Types => Enum.GetValues(typeof(FlightType)).Cast<FlightType>().Select(type =>
        {
            switch (type)
            {
                case FlightType.OneWay:
                    return "one-way flight";

                case FlightType.TwoWay:
                    return "return flight";

                default:
                    throw new NotImplementedException();
            }
        });

        public Visibility IsDepartureErrorMessageVisible(DateTimeOffset departureDate)
        {
            return IsDepartureDateValid(departureDate) ? Visibility.Collapsed : Visibility.Visible;
        }

        public Visibility IsReturnErrorMessageVisible(FlightType type, DateTimeOffset departureDate, DateTimeOffset returnDate)
        {
            return !IsReturnDateEnabled(type) || IsReturnDateValid(departureDate, returnDate) ? Visibility.Collapsed : Visibility.Visible;
        }

        public bool IsReturnDateEnabled(FlightType type)
        {
            return type == FlightType.TwoWay;
        }

        public bool IsBookable(FlightType type, DateTimeOffset departureDate, DateTimeOffset returnDate)
        {
            switch (type)
            {
                case FlightType.OneWay:
                    return IsDepartureDateValid(departureDate);

                case FlightType.TwoWay:
                    return IsReturnDateValid(departureDate, returnDate);

                default:
                    throw new NotImplementedException();
            }
        }

        private static bool IsDepartureDateValid(DateTimeOffset departureDate)
        {
            return departureDate.Date >= DateTimeOffset.Now.Date;
        }

        private static bool IsReturnDateValid(DateTimeOffset departureDate, DateTimeOffset returnDate)
        {
            return returnDate.Date >= departureDate.Date;
        }
    }

    public enum FlightType
    {
        OneWay = 0,
        TwoWay = 1
    }

    public class FlightTypeToIntConverter : IValueConverter
    {
        public object Convert(object value, Type targetType, object parameter, string language)
        {
            FlightType flightType = (FlightType)value;
            switch (flightType)
            {
                case FlightType.OneWay:
                    return 0;

                case FlightType.TwoWay:
                    return 1;

                default:
                    throw new NotImplementedException();
            }
        }

        public object ConvertBack(object value, Type targetType, object parameter, string language)
        {
            int flightType = (int)value;
            switch (flightType)
            {
                case 0:
                    return FlightType.OneWay;

                case 1:
                    return FlightType.TwoWay;

                default:
                    throw new NotImplementedException();
            }
        }
    }
}
