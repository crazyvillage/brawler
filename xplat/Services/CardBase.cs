using xplat.Data;

namespace xplat.Services;

public abstract class CardBase
{
    public virtual string Name { get; set; }
    public Card ToCard()
    {
        return new Card
        {
            Name = SetName(),
            IsCommander = SetIsCommander(),
            Colors = SetColors()
        };
    }

    protected abstract string SetName();
    protected abstract bool SetIsCommander();
    protected abstract int SetColors();
}