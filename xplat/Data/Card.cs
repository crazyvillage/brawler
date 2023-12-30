using SQLite;

namespace xplat.Data;

public class Card
{
    [PrimaryKey, AutoIncrement]
    public int Id { get; set; }
    public string Name { get; set; }
    public int Colors { get; set; }
    public bool IsCommander { get; set; }
}