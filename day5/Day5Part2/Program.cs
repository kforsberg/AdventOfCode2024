var fileText = File.ReadAllText("inputs/input.txt");
var split = fileText.Split("\r\n\r\n");
var orderingRules = BuildOrderingRules(split[0]);
var updates = BuildUpdates(split[1]);
var invalidUpdates = updates.Where(x => !IsUpdateValid(x, orderingRules));

var sortedUpdates = new List<List<int>>();
foreach (var update in invalidUpdates)
{
    update.Sort(new OrderRuleComparer(orderingRules));
    sortedUpdates.Add(update);
}

var sum = sortedUpdates.Sum(x => GetMiddleUpdate(x));

Console.WriteLine($"There are {invalidUpdates.Count()} valid updates");
Console.WriteLine($"The sum of the middles is {sum}");


List<PageOrderRule> BuildOrderingRules(string orderingText)
{
    var orderingRules = new List<PageOrderRule>();
    foreach (var orderRule in orderingText.Split("\n"))
    {
        orderingRules.Add(new PageOrderRule
        {
            First = int.Parse(orderRule.Split("|")[0]),
            Last = int.Parse(orderRule.Split("|")[1]),
        });
    }
    return orderingRules;
}

List<List<int>> BuildUpdates(string updatesText)
{
    var updates = new List<List<int>>();
    foreach (var updateText in updatesText.Split("\r\n"))
    {
        var update = new List<int>();
        foreach (var updateRule in updateText.Split(','))
        {
            update.Add(int.Parse(updateRule));
        }
        updates.Add(update);
    }
    return updates;
}

bool IsUpdateValid(List<int> updates, List<PageOrderRule> rules)
{
    return !rules.Any(rule => updates.Contains(rule.First) && updates.Contains(rule.Last)
        && updates.IndexOf(rule.First) >= updates.IndexOf(rule.Last));
}

int GetMiddleUpdate(List<int> updates)
{
    return updates[(int)Math.Floor(updates.Count() / 2.0)];
}

class PageOrderRule
{
    public required int First { get; set; }
    public required int Last { get; set; }
    public override string ToString()
    {
        return $"{First}|{Last}";
    }

}

class OrderRuleComparer : IComparer<int>
{
    private readonly List<PageOrderRule> rules;

    public OrderRuleComparer(List<PageOrderRule> rules)
    {
        this.rules = rules;
    }
    public int Compare(int x, int y)
    {
        if (rules.Select(r => r.ToString()).Contains($"{x}|{y}"))
        {
            return -1;
        }
        else if (rules.Select(r => r.ToString()).Contains($"{y}|{x}"))
        {
            return 1;
        }
        return 0;
    }
}