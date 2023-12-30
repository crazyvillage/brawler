using System.Text.Json.Serialization;

namespace xplat.Services;

public class ScryfallResponse
{
    [JsonPropertyName("has_more")]
    public bool HasMore { get; set; }
    
    [JsonPropertyName("next_page")]
    public string NextPage { get; set; }
    
    [JsonPropertyName("data")]
    public IEnumerable<ScryfallCard> Data { get; set; }
}