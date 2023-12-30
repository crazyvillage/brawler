using Refit;

namespace xplat.Services;

public interface IScryfallClient
{
    [Get("/cards/search?q=f:historic")]
    Task<ScryfallResponse> GetHistoricCards();
}