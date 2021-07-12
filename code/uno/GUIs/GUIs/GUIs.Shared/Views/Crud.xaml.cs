using GUIs.Common;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using Windows.UI.Xaml;
using Windows.UI.Xaml.Controls;

// The User Control item template is documented at https://go.microsoft.com/fwlink/?LinkId=234236

namespace GUIs.Views
{
    public sealed partial class Crud : UserControl
    {
        private Db db = new Db();
        public Signal<int> SelectedIndex = new Signal<int>(0);
        public Signal<string> Query = new Signal<string>("");
        public Signal<string> Forename = new Signal<string>("");
        public Signal<string> Surname = new Signal<string>("");
        private readonly List<ClientUser> users;
        public ObservableCollection<ClientUser> FilteredUsers;

        public Crud()
        {
            InitializeComponent();
            users = new List<ClientUser>
            {
                new ClientUser(db.Create("Hans", "Emil")),
                new ClientUser(db.Create("Max", "Mustermann")),
                new ClientUser(db.Create("Roman", "Tisch"))
            };
            FilteredUsers = new ObservableCollection<ClientUser>(new List<ClientUser>(users));
        }

        public void HandleQueryChanged(object sender, TextChangedEventArgs e)
        {
            Query.Value = ((TextBox)sender).Text;
            var query = Query.Value.ToLower();
            var newList = query.Length == 0 ? users : users.FindAll(user => user.Name.ToLower().Contains(query) || user.Surname.ToLower().Contains(query));
            FilteredUsers.Clear();
            ; foreach (var item in newList)
            {
                FilteredUsers.Add(item);
            }
        }

        public void HandleCreate(object sender, RoutedEventArgs e)
        {
            var user = new ClientUser(db.Create(Forename.Value, Surname.Value));
            users.Add(user);
            if (user.Name.Contains(Query.Value) || user.Surname.Contains(Query.Value))
            {
                FilteredUsers.Add(user);
            }
        }

        public void HandleUpdate(object sender, RoutedEventArgs e)
        {
            if (SelectedIndex.Value > -1)
            {
                var user = FilteredUsers[SelectedIndex.Value];
                user.UpdateFromBackend(db.Update(user.Id, Forename.Value, Surname.Value));
            }
        }

        public void HandleDelete(object sender, RoutedEventArgs e)
        {
            if (SelectedIndex.Value > -1)
            {
                var user = FilteredUsers[SelectedIndex.Value];
                db.Delete(user.Id);
                users.Remove(user);
                FilteredUsers.RemoveAt(SelectedIndex.Value);
            }
        }
    }

    public class ClientUser : Trackable
    {
        public readonly string Id;
        private string name;
        private string surname;

        public string Name { get => name; set { name = value; Track(nameof(Name)); } }
        public string Surname { get => surname; set { surname = value; Track(nameof(Surname)); } }

        public ClientUser(User user)
        {
            Id = user.Id;
            name = user.Name;
            surname = user.Surname;
        }

        public void UpdateFromBackend(User user)
        {
            Name = user.Name;
            Surname = user.Surname;
        }

        public static string Format(string name, string surname)
        {
            return string.Format("{0}, {1}", surname, name);
        }
    }

    public class Db
    {
        private int idCounter = 0;
        private List<User> users = new List<User>();

        public User Create(string name, string surname)
        {
            var user = new User(idCounter.ToString(), name, surname);
            users.Add(user);
            idCounter += 1;
            return user;
        }

        public User Update(string id, string name, string surname)
        {
            var index = users.FindIndex(u => id == u.Id);
            if (index > -1)
            {
                var user = users[index];
                user.Name = name;
                user.Surname = surname;
                return user;
            }
            return null;
        }

        public void Delete(string id)
        {
            var index = users.FindIndex(u => id == u.Id);
            if (index > -1)
            {
                users.RemoveAt(index);
            }
        }
    }

    public class User
    {
        public readonly string Id;
        public string Name;
        public string Surname;

        public User(string id, string name, string surname)
        {
            Id = id;
            Name = name;
            Surname = surname;
        }
    }
}
