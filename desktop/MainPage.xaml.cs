namespace Desktop;

public partial class MainPage : ContentPage
{
	int count = 0;

	public MainPage()
	{
		InitializeComponent();
	}

	private void OnCopyClicked(object sender, EventArgs e)
	{
		count++;

		if (count == 1)
			CopyBtn.Text = $"Clicked {count} time";
		else
			CopyBtn.Text = $"Clicked {count} times";

		SemanticScreenReader.Announce(CopyBtn.Text);
	}
}

