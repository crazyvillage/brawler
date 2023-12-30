using SQLite;

namespace xplat.Data;

public class BrawlerDatabase
{
        private SQLiteAsyncConnection _database;

        public BrawlerDatabase()
        {
        }

        private async Task Init()
        {
            if (_database is not null)
                return;

            _database = new SQLiteAsyncConnection(Constants.DatabasePath, Constants.Flags);
            var result = await _database.CreateTableAsync<Card>();
        }
}