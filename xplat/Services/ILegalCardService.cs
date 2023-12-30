namespace xplat.Services;

public interface ILegalCardService
{
    Task<IEnumerable<CardBase>> GetLegalCards();
}