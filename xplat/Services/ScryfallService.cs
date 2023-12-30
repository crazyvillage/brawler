using Refit;

namespace xplat.Services;

public class ScryfallService : ILegalCardService
{
    private readonly IScryfallClient _client = RestService.For<IScryfallClient>("https://api.scryfall.com");

    public async Task<IEnumerable<CardBase>> GetLegalCards()
    {
        var response = await _client.GetHistoricCards();
        return response.Data;
    }
}