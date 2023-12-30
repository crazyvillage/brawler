using System.Text.Json.Serialization;
using static System.Enum;

namespace xplat.Services;

public class ScryfallCard : CardBase
{
    [JsonPropertyName("name")]
    public override string Name { get; set; }
    
    [JsonPropertyName("type_line")]
    public string TypeLine { get; set; }
    
    [JsonPropertyName("colors")]
    public IEnumerable<string> Colors { get; set; }
    
    protected override string SetName()
    {
        return Name;
    }

    protected override bool SetIsCommander()
    {
        return TypeLine.StartsWith("Legendary Creature") || TypeLine.StartsWith("Legendary Planeswalker");
    }

    protected override int SetColors()
    {
        var n = 0;
        foreach (var c in Colors)
        {
            if (TryParse(c, true, out Colors color))
            {
                n += (int)color;
            }
        }

        return n;
    }
}