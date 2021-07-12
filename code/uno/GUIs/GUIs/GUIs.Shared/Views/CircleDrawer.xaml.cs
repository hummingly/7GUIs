using System.Collections.ObjectModel;
using Windows.UI.Xaml.Controls;
using Windows.UI.Xaml.Input;
using Windows.UI.Xaml;
using System.Collections.Generic;
using GUIs.Common;

// The User Control item template is documented at https://go.microsoft.com/fwlink/?LinkId=234236

namespace GUIs.Views
{
    public sealed partial class CircleDrawer : UserControl
    {
        public ObservableCollection<Circle> Circles = new ObservableCollection<Circle>();
        public CircleChanges Changes = new CircleChanges();
        private Circle selectedCircle = null;
        private float oldDiameter = -1.0f;
        private int idCounter = 0;

        public CircleDrawer()
        {
            InitializeComponent();
        }

        public void SelectCircle(XamlUICommand sender, ExecuteRequestedEventArgs args)
        {
            if (args.Parameter == null)
            {
                return;
            }
            int id = (int)args.Parameter;
            foreach (var circle in Circles)
            {
                if (id == circle.Id)
                {
                    selectedCircle = circle;
                    oldDiameter = circle.Diameter;
                    break;
                }
            }
        }

        public void CreateCircle(object sender, TappedRoutedEventArgs e)
        {
            if (sender == e.OriginalSource)
            {
                var position = e.GetPosition((UIElement)sender);
                var circle = new Circle(idCounter, position.X, position.Y, 30);
                Circles.Add(circle);
                Changes.Create(circle.ToData());
                e.Handled = true;
                idCounter += 1;
            }
        }

        public void Undo(object sender, RoutedEventArgs e)
        {
            var change = Changes.Undo();
            var id = change.Data.Id;
            if (change.Type == ChangeType.Create)
            {
                for (int i = 0; i < Circles.Count; i++)
                {
                    if (Circles[i].Id == id)
                    {
                        Circles.RemoveAt(i);
                        break;
                    }
                }
            }
            else if (change.Type == ChangeType.Update)
            {
                var lastChangeIndex = Changes.History.FindIndex(0, Changes.HistoryIndex + 1, c => c.Data.Id == id);
                if (lastChangeIndex > -1)
                {
                    var previousDiameter = Changes.History[lastChangeIndex].Data.Diameter;
                    foreach (var circle in Circles)
                    {
                        if (circle.Id == id)
                        {
                            circle.Diameter = previousDiameter;
                            break;
                        }
                    }
                }
            }
        }

        public void Redo(object sender, RoutedEventArgs e)
        {
            var change = Changes.Redo();
            if (change.Type == ChangeType.Create)
            {
                Circles.Add(new Circle(change.Data));
            }
            else if (change.Type == ChangeType.Update)
            {
                var id = change.Data.Id;
                foreach (var circle in Circles)
                {
                    if (circle.Id == id)
                    {
                        circle.Diameter = change.Data.Diameter;
                        break;
                    }
                }
            }
        }

        public void SaveCircle(object sender, object e)
        {
            if (oldDiameter != selectedCircle.Diameter)
            {
                var data = selectedCircle.ToData();
                Changes.Update(data);
            }
            oldDiameter = -1.0f;
            selectedCircle = null;
        }

        public static bool IsUndoEnabled(int historyIndex)
        {
            return historyIndex > -1;
        }

        public static bool IsRedoEnabled(int historyIndex, int changesCount)
        {
            return historyIndex != changesCount - 1;
        }
    }

    public class Circle : Trackable
    {
        private float diameter;

        public Circle(int id, double x, double y, float diameter)
        {
            Id = id;
            X = x;
            Y = y;
            this.diameter = diameter;
        }

        public Circle(CircleData data)
        {
            Id = data.Id;
            X = data.X;
            Y = data.Y;
            diameter = data.Diameter;
        }

        public int Id { get; }
        public double X { get; }
        public double Y { get; }
        public float Diameter
        {
            get => diameter;
            set
            {
                diameter = value;
                Track(nameof(Diameter));
            }
        }

        public CircleData ToData()
        {
            return new CircleData(Id, X, Y, Diameter);
        }
    }

    public class CircleChanges : Trackable
    {
        public readonly List<Change> History = new List<Change>();
        private int historyIndex = -1;

        public int HistoryIndex
        {
            get => historyIndex;
            set
            {
                historyIndex = value;
                Track(nameof(HistoryIndex));
            }
        }

        public void Create(CircleData data)
        {
            AddChange(ChangeType.Create, data);
        }

        public void Update(CircleData data)
        {
            AddChange(ChangeType.Update, data);
        }

        private void AddChange(ChangeType type, CircleData data)
        {
            var oldCount = History.Count;
            var currentChangesCount = HistoryIndex + 1;
            History.RemoveRange(currentChangesCount, History.Count - currentChangesCount);
            History.Add(new Change(type, data));
            HistoryIndex = History.Count - 1;
            if (oldCount != History.Count)
            {
                Track(nameof(History.Count));
            }
        }

        public Change Undo()
        {
            var change = History[HistoryIndex];
            HistoryIndex -= 1;
            return change;
        }

        public Change Redo()
        {
            HistoryIndex += 1;
            return History[HistoryIndex];
        }
    }


    public readonly struct CircleData
    {
        public CircleData(int id, double x, double y, float diameter)
        {
            Id = id;
            X = x;
            Y = y;
            Diameter = diameter;
        }

        public int Id { get; }
        public double X { get; }
        public double Y { get; }
        public float Diameter { get; }
    }

    public readonly struct Change
    {
        public Change(ChangeType type, CircleData data)
        {
            Type = type;
            Data = data;
        }

        public ChangeType Type { get; }
        public CircleData Data { get; }
    }

    public enum ChangeType
    {
        Create,
        Update
    }

    public static class Utility
    {
        public static CornerRadius GetRadiusFromDiameter(float diameter)
        {
            return new CornerRadius(diameter / 2.0f);
        }

        public static double GetOffset(double origin, float diameter)
        {
            return origin - (diameter / 2.0);
        }
    }
}
