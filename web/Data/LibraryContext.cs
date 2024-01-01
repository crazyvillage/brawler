using Microsoft.EntityFrameworkCore;

namespace web.Data;

public class LibraryContext : DbContext
{
    public DbSet<Card>? Cards { get; set; }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        optionsBuilder.UseNpgsql("Host=localhost;Database=brawler;Username=postgres;Password=m4gnum")
            .UseSnakeCaseNamingConvention();
    }
}

public class Card
{
    public int Id { get; set; }
    public string Name { get; set; }
    public int Colors { get; set; }
    public bool IsCommander { get; set; }
}