using xplat.Services;

namespace xplat;

public partial class MainPage : ContentPage
{
	private readonly ILegalCardService _legalCardService;
	
	public MainPage(ILegalCardService legalCardService)
	{
		_legalCardService = legalCardService;
		InitializeComponent();
	}

	private async void OnReloadClicked(object sender, EventArgs e)
	{
		var cards = await _legalCardService.GetLegalCards();
		deckLabel.Text = string.Join(',', cards.Select(c => c.Name));
	}
}

